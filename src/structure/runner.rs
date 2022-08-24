use std::cell::RefCell;
use crate::structure::common::*;
use std::collections::VecDeque;
use std::io::Read;
use std::process::Child;
use std::sync::mpsc;
use std::sync::mpsc::SendError;
use std::time::Duration;
use std::process::Command;

fn parse_command(text: impl Into<String>) -> Result<Command, String> {
    let command_and_args = shlex::split(&text.into())
        .ok_or("Command \"{command}\" is invalid!".to_string())?;
    let mut command = Command::new(&command_and_args[0]);
    command.args(&command_and_args[1..]);
    Ok(command)
}

pub struct CodeRunner {
    incoming: mpsc::Receiver<RunResponse>,
    outgoing: mpsc::Sender<RunRequest>,
}
impl CodeRunner {
    pub fn please_run(
        &self,
        examples: Vec<Example>,
        compile_script: String,
        run_script: String,
    ) -> Result<(), SendError<RunRequest>> {
        self.outgoing.send(RunRequest::PleaseRun(RunDetails {
            compile_script,
            run_script,
            examples,
        }))
    }

    pub fn please_stop(&self) -> Result<(), SendError<RunRequest>> {
        self.outgoing.send(RunRequest::PleaseStop)
    }

    pub fn get_updates(&self) -> Vec<RunResponse> {
        self.incoming.try_iter().collect()
    }
}

impl Default for CodeRunner {
    fn default() -> Self {
        let (to_main, from_runner) = mpsc::channel();
        let (to_runner, from_main) = mpsc::channel();

        std::thread::spawn(move || {
            RemoteRunner::new(from_main, to_main).run();
        });

        Self {
            incoming: from_runner,
            outgoing: to_runner,
        }
    }
}

#[derive(Debug)]
struct RunnableTestCase {
    pub id: usize,
    pub command_template: String,
    pub arg: String,
    pub expected_stdout: String,

    stdout: RefCell<String>,
    stderr: RefCell<String>,
    process: Option<std::process::Child>,
    start_time: Option<std::time::Instant>,
}

impl RunnableTestCase {
    pub fn new(id: usize, command_template: String, arg: String, expected_stdout: String) -> Self {
        Self {
            id,
            command_template: command_template,
            arg: arg,
            stdout: RefCell::default(),
            stderr: RefCell::default(),
            process: None,
            start_time: None,
            expected_stdout,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let process = parse_command(format!("{} {}", self.command_template, self.arg))
            .map_err(|err| format!("{}", err))?
            .spawn()
            .map_err(|err| format!("{}", err))?;

        self.process = Some(process);
        self.start_time = Some(std::time::Instant::now());
        Ok(())
    }

    pub fn has_started(&self) -> bool {
        self.process.is_some()
    }

    pub fn has_finished(&mut self) -> bool {
        match &mut self.process {
            Some(process) => match process.try_wait() {
                Ok(Some(_exit_status)) => true,
                Err(_err) => true,
                Ok(None) => false,
            },
            None => false,
        }
    }

    pub fn kill(&mut self) -> Result<(), std::io::Error> {
        self.process
            .as_mut()
            .unwrap_or_else(|| panic!("Evaluate called before process finished."))
            .kill()
    }

    fn get_results(self) -> (TestCaseStatus, Duration) {
        let Self { id, command_template, arg, expected_stdout, stdout, stderr, process, start_time } = self;
        let time = start_time.unwrap().elapsed();
        let mut process = process.unwrap_or_else(|| {
            panic!("RunnableTestCase::get_results called before process finished.")
        });
        let exit_status = process.try_wait().unwrap();

        match exit_status {
            None => panic!("Evaluate called before process finished."),
            Some(status) => {
                if status.success() {
                    let stdout = Self::get_stdout(process);
                    if stdout == expected_stdout {
                        return (TestCaseStatus::Pass { actual: stdout }, time);
                    }
                    return (
                        TestCaseStatus::Fail {
                            expected: expected_stdout.clone(),
                            actual: stdout,
                        },
                        time,
                    );
                }

                let stderr = Self::get_stderr(process);
                return (TestCaseStatus::Err { err_msg: stderr }, time);
            }
        }
    }

    fn get_stderr(process: Child) -> String {
        let mut stderr = String::new();
        process.stderr.unwrap().read_to_string(&mut stderr);
        stderr
    }

    fn get_stdout(process: Child) -> String {
        let mut stdout = String::new();
        process.stdout.unwrap().read_to_string(&mut stdout);
        stdout
    }
}

struct RemoteRunner {
    incoming: mpsc::Receiver<RunRequest>,
    outgoing: mpsc::Sender<RunResponse>,
    to_run: VecDeque<RunnableTestCase>,
}

impl RemoteRunner {
    pub fn new(incoming: mpsc::Receiver<RunRequest>, outgoing: mpsc::Sender<RunResponse>) -> Self {
        Self {
            incoming,
            outgoing,
            to_run: VecDeque::new(),
        }
    }

    pub fn run(mut self) {
        loop {
            self.try_handle_next_request();
            self.continue_running_last_request();
        }
    }

    fn try_handle_next_request(&mut self) -> Option<()> {
        let new_request = self.receive_new_run_request()?;

        self.abort_curr_run();

        if let RunRequest::PleaseRun(run_details) = new_request {
            let compilation_status = self.setup_new_run(run_details);
            if let Err(err_msg) = compilation_status {
                let status = TestCaseStatus::Err { err_msg };
                self.notify(0, status);
            }
        }
        Some(())
    }

    fn notify(&self, id: usize, status: TestCaseStatus) -> Result<(), SendError<RunResponse>> {
        let response = RunResponse { id, status };
        self.outgoing.send(response)
    }

    fn continue_running_last_request(&mut self) -> Option<()> {
        let mut current_test_case = self.to_run.pop_front()?;

        if !current_test_case.has_started() {
            let status = match current_test_case.start() {
                Ok(()) => TestCaseStatus::Running,
                Err(err) => {
                    let err_msg = format!("{}", err);
                    TestCaseStatus::Err { err_msg } 
                }
            };
            self.notify(current_test_case.id, status).unwrap();
        }

        if current_test_case.has_finished() {
            let id: usize = current_test_case.id.clone();
            let (status, time_completed) = current_test_case.get_results();
            self.notify(id, status).unwrap();
        } else {
            self.to_run.push_front(current_test_case);
        }

        Some(())
    }

    fn abort_curr_run(&mut self) {
        for test_case in &self.to_run {
            self.notify_cancelled(test_case)
                .unwrap_or_else(|_| panic!("could not send notification duting abort curr run"));
        }
        self.to_run = VecDeque::new();
    }

    fn notify_cancelled(&self, test_case: &RunnableTestCase) -> Result<(), SendError<RunResponse>> {
        self.outgoing.send(RunResponse {
            id: test_case.id,
            status: TestCaseStatus::Cancelled,
        })
    }

    fn setup_new_run(&mut self, run_details: RunDetails) -> Result<String, String> {
        let RunDetails {
            compile_script,
            run_script,
            examples,
        } = run_details;
        self.to_run = examples
            .into_iter()
            .enumerate()
            .map(|(id, ex)| RunnableTestCase::new(id, run_script.clone(), ex.input, ex.output))
            .collect();
        self.compile(compile_script)
    }

    fn compile(&self, command: String) -> Result<String, String> {
        let mut process = parse_command(command)?;
        process.output()
            .map_err(|err| format!("{}", err))
            .map(|ok| "Compilation success!".to_string())
    }

    fn receive_new_run_request(&self) -> Option<RunRequest> {
        self.incoming.recv().ok()
    }
}
