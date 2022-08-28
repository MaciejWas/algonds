use std::fmt::Display;
use std::process::Command;

mod code_runner;
mod remote_runner;
mod runnable_test_case;
mod test_case_io;

pub use code_runner::CodeRunner;
pub use remote_runner::RemoteRunner;
pub use runnable_test_case::RunnableTestCase;
pub use test_case_io::TestCaseIO;

fn to_string<T: Display>(thing: T) -> String {
    format!("{}", thing)
}

fn parse_command(text: impl Into<String>) -> Result<Command, String> {
    let whole_command = text.into();
    if whole_command.is_empty() {
        return Err("As of now, all compile/run command must noe be empty".into());
    }

    let command_and_args =
        shlex::split(&whole_command).ok_or("Failed to parse command:\"{command}\"".to_string())?;
    let mut command = Command::new(&command_and_args[0]);
    command.args(&command_and_args[1..]);
    Ok(command)
}
