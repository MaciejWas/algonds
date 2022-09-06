use crate::application::common::TestCaseStatus;
use crate::application::test_runner::to_string;
use crate::application::test_runner::RemoteRunner;
use crate::application::RunDetails;
use crate::application::RunRequest;
use crate::application::RunResponse;
use crate::application::TestCase;
use std::sync::mpsc::SendError;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

#[derive(Debug)]
pub struct CodeRunner {
    handle: std::thread::JoinHandle<std::result::Result<(), String>>,
    incoming: Receiver<RunResponse>,
    outgoing: Sender<RunRequest>,
}
impl CodeRunner {
    pub fn please_run(
        &self,
        test_cases: Vec<TestCase>,
        compile_script: String,
        run_script: String,
    ) -> Result<(), SendError<RunRequest>> {
        self.outgoing.send(RunRequest::PleaseRun(RunDetails {
            compile_script,
            run_script,
            test_cases,
        }))
    }

    fn check_thread(&self) -> Result<(), String> {
        if self.handle.is_finished() {
            return Err("Thread which runs the test cases has died".into());
        }
        Ok(())
    }

    pub fn please_stop(&self) -> Result<(), String> {
        self.outgoing
            .send(RunRequest::PleaseStop)
            .map_err(to_string)
    }

    pub fn get_updates(&self) -> Vec<RunResponse> {
        if let Err(err_msg) = self.check_thread() {
            let status = TestCaseStatus::Err { err_msg };
            return vec![RunResponse { id: 0, status }];
        }

        let mut updates = Vec::new();
        while let Ok(response) = self.incoming.recv_timeout(Duration::from_millis(10)) {
            updates.push(response)
        }
        updates
    }
}

impl Default for CodeRunner {
    fn default() -> Self {
        let (to_main, from_runner) = channel();
        let (to_runner, from_main) = channel();

        let mut runner = RemoteRunner::new(from_main, to_main);

        let handle = std::thread::spawn(move || runner.run());

        Self {
            handle,
            incoming: from_runner,
            outgoing: to_runner,
        }
    }
}
