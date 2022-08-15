use std::time::Duration;
use std::process::ExitStatus;
use std::cell::Cell;
use std::rc::Rc;
use crate::structure::common::*;
use std::sync::mpsc;
use std::process::ChildStdout;
use std::fs::read_to_string;
use std::io::Read;

struct CodeRunner {
    incoming: mpsc::Receiver<RunResponse>,
    outgoing:  mpsc::Sender<RunRequest>
} impl CodeRunner {
    pub fn new() -> Self {
        let (to_main, from_runner) = mpsc::channel();
        let (to_runner, from_main) = mpsc::channel();

        std::thread::spawn(move || {
            RemoteRunner::new(from_main, to_main).run();
        });

        Self { incoming: from_runner, outgoing: to_runner }
    }
}

struct RunnableTestCase {
    command_template: String,
    arg: String,
    expected_stdout: String,
    process: Option<std::process::Child>,
    start_time: Option<std::time::Instant>,
} impl RunnableTestCase {
    pub fn new(command_template: String, arg: String, expected_stdout: String) -> Self {
        Self { command_template: command_template, arg: arg, process: None, start_time: None, expected_stdout }
    }

    pub fn begin(&mut self) -> Result<(), std::io::Error> {
        let process = std::process::Command::new(self.command_template).arg(self.arg);
        self.process = Some( process.spawn()? );
        self.start_time = Some( std::time::Instant::now() );
        Ok(())
    }

    pub fn has_started(&self) -> bool {
        self.start_time.is_some()
    }

    pub fn has_finished(&self) -> bool {
        match self.process {
            Some(process) => match process.try_wait() {
                Ok(Some(exit_status)) => true,
                Err(err) => true,
                Ok(None) => false,
            }
            None => panic!("RunnableTestCase::try_finish should not be called before RunnableTestCase::begin :(")
        }
    }

    pub fn try_finish(&self) {todo!()}

    pub fn kill(&mut self) {
        let process = self.process
            .unwrap_or_else(|| panic!("Evaluate called before process finished."));
        process.kill();
    }

    fn get_results(&self) -> (ExampleStatus, Duration) {
        let time = self.start_time.unwrap().elapsed();
        let process = self.process
            .unwrap_or_else(|| panic!("Evaluate called before process finished."));

        let exit_status = process.try_wait().unwrap();

        match exit_status {
            None => panic!("Evaluate called before process finished."),
            Some(status) => {
                if status.success() {
                    let mut stdout = String::new();
                    process.stdout.unwrap().read_to_string(&mut stdout);
                    if stdout == self.expected_stdout {
                        return (ExampleStatus::Pass, time)
                    }
                    return (ExampleStatus::Fail, time)
                }


                let mut stderr = String::new();
                process.stderr.unwrap().read_to_string(&mut stderr);

                return (ExampleStatus::Error(stderr), time)
            }
        }



    }
}

impl Drop for RunnableTestCase {
    fn drop(&mut self) { 
        if self.process.is_some() {
            self.kill()
        }
    }
}

struct RemoteRunner {
    incoming: mpsc::Receiver<RunRequest>,
    outgoing: mpsc::Sender<RunResponse>,
    to_run: Vec<RunnableTestCase>,
}

impl RemoteRunner {
    pub fn new(incoming: mpsc::Receiver<RunRequest>, outgoing: mpsc::Sender<RunResponse>) -> Self {
        Self { incoming, outgoing, to_run: Vec::new() }
    }

    pub fn run(mut self) {
        loop {
            if let Some(req) = self.receive_new_run_request() {
                match req {
                    RunRequest::PleaseRun(run_details) => {
                        self.abort_curr_run();
                        self.setup_new_run(run_details)
                    }
                    RunRequest::PleaseStop => self.abort_curr_run()
                }
            }

            self.continue_running();
        }
    }

    fn continue_running(&mut self) {
        match self.to_run.first_mut() {
            None => {}
            Some(test_case) => {
                if !test_case.has_started() {
                    test_case.begin();
                }

                if test_case.has_finished() {
                    let (status, time_completed) = test_case.get_results();
                    self.notify_finished(&test_case, status, time_completed);
                }
            }
        };
    }

    fn finish_with_result(&mut self, result: Result<(), std::boxed::Box<dyn std::any::Any + std::marker::Send>>) {
        todo!()
    }

    fn start_next_example(&mut self) {
        todo!()
    }

    fn abort_curr_run(&mut self) {
        for test_case in self.to_run {
            self.notify_cancelled(&test_case);
        }
        self.to_run = Vec::new();
    }

    fn notify_cancelled(&self, test_case: &RunnableTestCase) {
        todo!()
    }

    fn setup_new_run(&self, run_details: RunDetails) {
        let RunDetails { compile_script, run_script, examples } = run_details;

    }

    fn compile(&self) -> Option<std::io::Error> {
        let compile_output = std::process::Command::new(self.running?.compile_script).output();
        match compile_output {
            Err(e) => Some(e),
            Ok(_) => None,
        }
    }

    fn notify_started(&self, run_details: &RunDetails) {

    }

    fn notify_finished(&self, test_case: &RunnableTestCase, status: ExampleStatus, time_completed: Duration) {
            
    }

    fn receive_new_run_request(&self) -> Option<RunRequest> {
        todo!()
    }

    fn set_run_details(&mut self, run_details: RunDetails) {
        self.running = Some(run_details);
    }

    fn run_next_example(&self)  {
        self.notify_started();
                let result: ExampleStatus = self.run_next_example(&run_details);
        self.notify_finished();


        let RunDetails { compile_script, run_script, examples } = &run_details;


        match self.last_run_reqest? {
            PleaseRun { compile_script, run_script, examples } => {                


                let example = examples.get(self.running)?;
                let run_output = std::process::Command::new(run_script.replace("{args}", &example.input)).output();

                None
            },
            PleaseStop => return None
        }
    }
}