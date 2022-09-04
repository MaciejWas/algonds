use std::{
    process::Child,
    time::{Duration, Instant},
};

use super::{parse_command, TestCaseIO};
use crate::application::common::TestCaseStatus;

const ZERO_SECS: Duration = Duration::from_secs(0);

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[derive(Debug)]
struct RunnableTestCaseInner {
    pub io: TestCaseIO,
    pub process: Child,
    pub start_time: Instant,
}

#[derive(Debug)]
pub struct RunnableTestCase {
    pub id: usize,
    pub command_template: String,
    pub arg: String,
    pub expected_stdout: String,

    error: Option<String>,
    inner: Option<RunnableTestCaseInner>,
}

impl RunnableTestCase {
    pub fn new(id: usize, command_template: String, arg: String, expected_stdout: String) -> Self {
        Self {
            id,
            command_template: command_template,
            arg: arg,
            expected_stdout,

            error: None,
            inner: None,
        }
    }

    pub fn start(&mut self) {
        let result = self.start_inner();
        self.error = result.err();
    }

    fn start_inner(&mut self) -> Result<(), String> {
        let io = TestCaseIO::new()?;
        let (stdout, stderr) = io.get_io()?;
        
        let process = parse_command(format!("{} {}", self.command_template, self.arg))
            .map_err(|err| format!("{}", err))?
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

    fn error_result(err_msg: impl Into<String>) -> (TestCaseStatus, Duration) {
        let err_msg = err_msg.into();
        (TestCaseStatus::Err { err_msg }, ZERO_SECS)
    }

    pub fn get_results(self) -> (TestCaseStatus, Duration) {
        let Self {
            id: _,
            command_template: _,
            arg: _,
            expected_stdout,
            error,
            inner,
        } = self;

        if let Some(err_msg) = error {
            return Self::error_result(err_msg + &"(While handling the test case)");
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

        return match exit_status {
            None => Self::error_result("Process has not finished but shuld have"),
            Some(status) => {
                if status.success() {
                    let stdout = match io.get_stdout() {
                        Ok(stdout) => stdout,
                        Err(err_msg) => {
                            return (TestCaseStatus::Err { err_msg: err_msg + &"(while cheching stdout)" }, time)
                        }
                    };
                    if remove_whitespace(&stdout) == remove_whitespace(&expected_stdout) {
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

                let err_msg = match io.get_stderr() {
                    Ok(stderr) => stderr,
                    Err(err_msg) => err_msg,
                };

                return (TestCaseStatus::Err { err_msg }, time)
            }
        };
    }
}
