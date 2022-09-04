use std::{fs::File, io::Read, process::Stdio};

use super::to_string;

const STDOUT_LOC: &str = "/tmp/algonds_tc_stdout.txt";
const STDERR_LOC: &str = "/tmp/algonds_tc_stderr.txt";

/// This struct keeps track of IO of a test case
#[derive(Debug)]
pub struct TestCaseIO {
    stderr_file: File,
    stdout_file: File,
}

impl TestCaseIO {
    pub fn new() -> Result<Self, String> {
        let stdout_file = File::create(STDOUT_LOC).map_err(to_string)?;
        let stderr_file = File::create(STDERR_LOC).map_err(to_string)?;

        Ok(Self {
            stderr_file,
            stdout_file,
        })
    }

    pub fn get_io(&self) -> Result<(Stdio, Stdio), String> {
        let stdout: Stdio = self.stdout_file.try_clone().map_err(to_string)?.into();
        let stderr: Stdio = self.stderr_file.try_clone().map_err(to_string)?.into();

        Ok((stdout, stderr))
    }

    pub fn get_stdout(&mut self) -> Result<String, String> {
        let mut buf = String::new();
        File::open(STDOUT_LOC)
            .unwrap()
            .read_to_string(&mut buf)
            .map_err(to_string)?;
        Ok(buf)
    }

    pub fn get_stderr(&mut self) -> Result<String, String> {
        let mut buf = String::new();
        File::open(STDERR_LOC)
            .unwrap()
            .read_to_string(&mut buf)
            .map_err(to_string)?;
        Ok(buf)
    }
}
