use crate::application::test_runner::parse_command;
use crate::application::test_runner::RunnableTestCase;
use crate::application::RunDetails;
use crate::application::RunRequest;
use crate::application::RunResponse;
use crate::application::TestCaseStatus;
use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, Sender};

use super::to_string;

pub struct RemoteRunner {
    incoming: Receiver<RunRequest>,
    outgoing: Sender<RunResponse>,
    to_run: VecDeque<RunnableTestCase>,
}

impl RemoteRunner {
    pub fn new(incoming: Receiver<RunRequest>, outgoing: Sender<RunResponse>) -> Self {
        Self {
            incoming,
            outgoing,
            to_run: VecDeque::new(),
        }
    }

    pub fn run(mut self) -> Result<(), String> {
        loop {
            self.try_handle_next_request()?;
            self.continue_running_last_request()?;
        }
    }

    fn try_handle_next_request(&mut self) -> Result<(), String>{
        let new_request = match self.receive_new_run_request() {
            Some(req) => req,
            None => return Ok(())
        };

        self.abort_curr_run()?;

        if let RunRequest::PleaseRun(run_details) = new_request {
            let compilation_status = self.setup_new_run(run_details);
            if let Err(err_msg) = compilation_status {
                let status = TestCaseStatus::Err { err_msg };
                self.notify(0, status)?;
            }
        }
        Ok(())
    }

    fn notify(&self, id: usize, status: TestCaseStatus) -> Result<(), String> {
        let response = RunResponse { id, status };
        self.outgoing.send(response).map_err(to_string)
    }

    fn continue_running_last_request(&mut self) -> Result<(), String> {
        let mut current_test_case: RunnableTestCase = match self.to_run.pop_front() {
            Some(tc) => tc,
            None => return Ok(()),
        };

        if !current_test_case.has_started() {
            current_test_case.start();
            self.notify(current_test_case.id, TestCaseStatus::Running)?;
        }

        if current_test_case.has_finished() {
            let id: usize = current_test_case.id.clone();
            let (status, _time_completed) = current_test_case.get_results();
            self.notify(id, status)?;
        } else {
            self.to_run.push_front(current_test_case);
        }

        Ok(())
    }

    fn abort_curr_run(&mut self) -> Result<(), String> {
        for test_case in &self.to_run {
            self.notify_cancelled(test_case)?;
        }
        self.to_run = VecDeque::new();
        Ok(())
    }

    fn notify_cancelled(&self, test_case: &RunnableTestCase) -> Result<(), String> {
        self.notify(test_case.id, TestCaseStatus::Cancelled)
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
        match process.output() {
            Ok(_) => Ok("Compilation success!".to_string()),
            Err(e) => Err(format!("{}", e))
        }
    }

    fn receive_new_run_request(&self) -> Option<RunRequest> {
        self.incoming.recv().ok()
    }
}
