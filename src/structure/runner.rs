use crate::structure::common::*;
use std::collections::VecDeque;
use std::io::Read;
use std::process::Child;
use std::sync::mpsc;
use std::sync::mpsc::SendError;
use std::time::Duration;

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
    process: Option<std::process::Child>,
    start_time: Option<std::time::Instant>,
}

impl RunnableTestCase {
    pub fn new(id: usize, command_template: String, arg: String, expected_stdout: String) -> Self {
        Self {
            id,
            command_template: command_template,
            arg: arg,
            process: None,
            start_time: None,
            expected_stdout,
        }
    }

    pub fn start(&mut self) -> Result<(), std::io::Error> {
        let process = std::process::Command::new(&self.command_template)
            .arg(&self.arg)
            .spawn()?;
        self.process = Some(process);
        self.start_time = Some(std::time::Instant::now());
        Ok(())
    }

    pub fn has_started(&self) -> bool {
        self.start_time.is_some()
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

    fn get_results(&mut self) -> (TestCaseStatus, Duration) {
        let time = self.start_time.unwrap().elapsed();
        let process = self.process.as_mut().unwrap_or_else(|| {
            panic!("RunnableTestCase::get_results called before process finished.")
        });
        let exit_status = process.try_wait().unwrap();

        match exit_status {
            None => panic!("Evaluate called before process finished."),
            Some(status) => {
                if status.success() {
                    let stdout = Self::get_stdout(process);
                    if stdout == self.expected_stdout {
                        return (TestCaseStatus::Pass { actual: stdout }, time);
                    }
                    return (
                        TestCaseStatus::Fail {
                            expected: self.expected_stdout.clone(),
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

    fn get_stderr(process: &mut Child) -> String {
        let mut stderr = String::new();
        process.stderr.as_mut().unwrap().read_to_string(&mut stderr);
        stderr
    }

    fn get_stdout(process: &mut Child) -> String {
        let mut stdout = String::new();
        process.stdout.as_mut().unwrap().read_to_string(&mut stdout);
        stdout
    }
}

impl Drop for RunnableTestCase {
    fn drop(&mut self) {
        if self.process.is_some() {
            self.kill().unwrap();
        }
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
            self.handle_next_request();
            self.continue_running();
        }
    }

    fn handle_next_request(&mut self) -> Option<()> {
        let new_request = self.receive_new_run_request()?;

        self.abort_curr_run();

        if let RunRequest::PleaseRun(run_details) = new_request {
            let compilation_status = self.setup_new_run(run_details);
            if let Err(err_msg) = compilation_status {
                let result = TestCaseStatus::Err { err_msg };
                self.outgoing.send(RunResponse { id: 0, result }).unwrap();
            }
        }
        Some(())
    }

    fn continue_running(&mut self) -> Option<()> {
        let current_test_case = self.to_run.front_mut()?;

        if !current_test_case.has_started() {
            current_test_case.start();
            self.outgoing.send( RunResponse { id: current_test_case.id, result: TestCaseStatus::Running } ).unwrap();
            return Some(());
        }

        if current_test_case.has_finished() {
            println!("heyy finished");
            let mut finished_test_case = self.to_run.pop_front()?;
            let (status, time_completed) = finished_test_case.get_results();
            self.notify_finished(&finished_test_case, status, time_completed);
            return Some(());
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
            result: TestCaseStatus::Cancelled,
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
        let command_and_args =
            shlex::split(&command).ok_or("command \"{command}\" is invalid!".to_string())?;
        let main_command = command_and_args
            .first()
            .ok_or("Compile script is empty!".to_string())?;
        let args = command_and_args.iter().skip(1);

        let compile_output = std::process::Command::new(&main_command)
            .args(args)
            .output();

        compile_output
            .map_err(|err| format!("{}", err))
            .map(|ok| "Compilation success!".to_string())
    }

    fn notify_finished(
        &self,
        test_case: &RunnableTestCase,
        status: TestCaseStatus,
        _time_completed: Duration,
    ) -> Result<(), SendError<RunResponse>> {
        self.outgoing.send(RunResponse {
            id: test_case.id,
            result: status,
        })
    }

    fn receive_new_run_request(&self) -> Option<RunRequest> {
        self.incoming.recv().ok()
    }
}
