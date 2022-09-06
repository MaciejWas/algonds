use std::{
    process::Child,
    time::Instant,
};

use super::{parse_command, TestCaseIO};
use crate::application::common::TestCaseStatus;

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[derive(Debug)]
struct RunnableTestCaseInner {
    pub io: TestCaseIO,
    pub process: Child,
    pub start_time: Instant,
}

impl RunnableTestCaseInner {
    fn kill(&mut self) -> bool {
        self.process.kill().is_ok()
    }
}

#[derive(Debug)]
pub struct RunnableTestCase {
    pub id: usize,
    pub complexity: u32,
    pub command_template: String,
    pub arg: String,
    pub expected_stdout: String,

    error: Option<String>,
    inner: Option<RunnableTestCaseInner>,
}

impl RunnableTestCase {
    pub fn new(id: usize, complexity: u32, command_template: String, arg: String, expected_stdout: String) -> Self {
        Self {
            id,
            command_template,
            arg,
            expected_stdout,
            complexity,

            error: None,
            inner: None,
        }
    }

    pub fn start(&mut self) {
        let result = self.start_inner();
        self.error = result.err();
    }

    pub fn kill(self) {
        self.inner.map(|mut i| i.kill());
    }

    fn start_inner(&mut self) -> Result<(), String> {
        let io = TestCaseIO::new()?;
        let (stdout, stderr) = io.get_io()?;

        let process = parse_command(format!("{} {}", self.command_template, self.arg))?
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .map_err(|err| format!("{}", err))?;

        let process = process;
        let start_time = Instant::now();

        self.inner = Some(RunnableTestCaseInner {
            io,
            process,
            start_time,
        });
        Ok(())
    }

    pub fn has_started(&self) -> bool {
        self.inner.is_some()
    }

    pub fn has_finished(&mut self) -> bool {
        if self.error.is_some() {
            return true;
        }

        if let Some(inner) = &mut self.inner {
            let res = match inner.process.try_wait() {
                Ok(Some(_)) => true,
                Ok(None) => false,
                Err(_err) => true,
            };

            return res;
        }

        false
    }

    fn error_result(err_msg: impl Into<String>) -> TestCaseStatus {
        let err_msg = err_msg.into();
        TestCaseStatus::Err { err_msg }
    }

    pub fn get_results(self) -> TestCaseStatus {
        let Self {
            id: _,
            command_template: _,
            arg: _,
            complexity,
            expected_stdout,
            error,
            inner,
        } = self;

        if let Some(err_msg) = error {
            return Self::error_result(err_msg +"(While handling the test case)");
        };

        let RunnableTestCaseInner {
            mut io,
            mut process,
            start_time,
        } = match inner {
            Some(inner) => inner,
            None => return Self::error_result("Process has not even started."),
        };

        let time = start_time.elapsed();
        let exit_status = match process.try_wait() {
            Ok(status) => status,
            Err(_) => return Self::error_result("Error checking exit status."),
        };

        match exit_status {
            None => Self::error_result("Process has not finished but shuld have"),
            Some(status) => {
                if status.success() {
                    let stdout = match io.get_stdout() {
                        Ok(stdout) => stdout,
                        Err(err_msg) => return TestCaseStatus::Err { err_msg: err_msg + "(while cheching stdout)"}
                    };

                    if remove_whitespace(&stdout) == remove_whitespace(&expected_stdout) {
                        return TestCaseStatus::Pass { time, complexity };
                    }

                    return TestCaseStatus::Fail {
                            expected: expected_stdout,
                            actual: stdout,
                            time,
                            complexity
                        }
                }

                let err_msg = match io.get_stderr() {
                    Ok(stderr) => stderr,
                    Err(err_msg) => err_msg,
                };

                TestCaseStatus::Err { err_msg }
            }
        }
    }
}
