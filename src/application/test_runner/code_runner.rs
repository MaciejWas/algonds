use std::time::Duration;
use crate::application::test_runner::RemoteRunner;
use crate::application::Example;
use crate::application::RunDetails;
use crate::application::RunRequest;
use crate::application::RunResponse;
use std::sync::mpsc::SendError;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct CodeRunner {
    handle: std::thread::JoinHandle<std::result::Result<(), String>>,
    incoming: Receiver<RunResponse>,
    outgoing: Sender<RunRequest>,
}
impl CodeRunner {
    pub fn please_run(
        &self,
        examples: Vec<Example>,
        compile_script: String,
        run_script: String,
    ) -> Result<(), SendError<RunRequest>> {
        if self.handle.is_finished() {
            panic!("Shit!!");
        }
        self.outgoing.send(RunRequest::PleaseRun(RunDetails {
            compile_script,
            run_script,
            examples,
        }))
    }

    pub fn please_stop(&self) -> Result<(), SendError<RunRequest>> {
        if self.handle.is_finished() {
            panic!("Shit!!");
        }
        self.outgoing.send(RunRequest::PleaseStop)
    }

    pub fn get_updates(&self) -> Vec<RunResponse> {
        if self.handle.is_finished() {
            panic!("Shit!!");
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

        let handle = std::thread::spawn(move || {
            RemoteRunner::new(from_main, to_main).run()
        });

        Self {
            handle,
            incoming: from_runner,
            outgoing: to_runner,
        }
    }
}
