use std::time::Duration;
use serde::{Deserialize, Serialize};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Next,
    Previous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    Select,
    Solve,
    Help,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ProblemDataTab {
    TestCases,
    Commands,
    Details,
    Performance
}

impl Default for ProblemDataTab {
    fn default() -> Self {
        Self::Commands
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::Select
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Problem {
    pub name: String,
    pub statement: String,
    pub test_cases: Vec<TestCase>,
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestCase {
    pub id: usize,
    pub complexity: u32,
    pub input: String,
    pub output: String,
    pub is_stress_test: bool,
}

#[memoize::memoize]
fn into_span_inner(tcs: TestCaseStatus) -> Span<'static> {
    let text = match &tcs {
        TestCaseStatus::Pass { .. } => "🗹 Passed",
        TestCaseStatus::Fail { .. } => "🗷 Failed",
        TestCaseStatus::Running => "⌛ Running",
        TestCaseStatus::Cancelled => "⚠ Cancelled",
        TestCaseStatus::NotRun => "🯄 Not Run",
        TestCaseStatus::Err { .. } => "‼ Error",
    };
    let style = match &tcs {
        TestCaseStatus::Pass { .. } => Style::default().fg(Color::Green),
        TestCaseStatus::Fail { .. } | TestCaseStatus::Err { .. } => Style::default().fg(Color::Red),
        TestCaseStatus::Cancelled | TestCaseStatus::Running => Style::default().fg(Color::Yellow),
        TestCaseStatus::NotRun => Style::default().fg(Color::Gray),
    };
    Span::styled(text, style)
}

impl TestCaseStatus {
    pub fn into_detailed<'a>(self) -> Vec<Spans<'a>> {
        match self {
            Self::Pass { .. } => vec![Spans::from("Test case passed!")],
            Self::Fail { expected, actual, time: _, complexity: _ } => {
                vec![
                    Spans::from(Span::styled(
                        "Test failed!",
                        Style::default().fg(Color::Red),
                    )),
                    Spans::from("Expected: ".to_string() + &expected),
                    Spans::from("Actual:   ".to_string() + &actual),
                ]
            }
            Self::Running => vec![Spans::from(
                "Test case is still running... (stderr/out directed to /tmp/algonds_stderr/out",
            )],
            Self::Cancelled => vec![Spans::from("Test case was cancelled")],
            Self::NotRun => vec![Spans::from("Test case was not yet run")],
            Self::Err { err_msg } => vec![
                Spans::from("The following error occured:"),
                Spans::from(err_msg),
            ],
        }
    }

    pub fn into_span(self) -> Span<'static> {
        into_span_inner(self)
    }
}

impl Default for TestCaseStatus {
    fn default() -> Self {
        Self::NotRun
    }
}

impl From<&Difficulty> for Span<'_> {
    fn from(diff: &Difficulty) -> Self {
        match diff {
            Difficulty::Easy => Span::styled("Easy  ", Style::default().fg(Color::Green)),
            Difficulty::Medium => Span::styled("Medium", Style::default().fg(Color::LightYellow)),
            Difficulty::Hard => Span::styled("Hard  ", Style::default().fg(Color::Red)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputField {
    CompileCommand,
    RunCommand,
}

#[derive(Clone)]
pub enum RunRequest {
    PleaseRun(RunDetails),
    PleaseStop,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TestCaseStatus {
    Pass { time: Duration, complexity: u32 },
    Fail { expected: String, actual: String, time: Duration, complexity: u32 },
    Err { err_msg: String },
    Cancelled,
    Running,
    NotRun,
}

#[derive(Debug, Clone)]
pub struct RunResponse {
    pub id: usize,
    pub status: TestCaseStatus,
}

#[derive(Clone)]
pub struct RunDetails {
    pub compile_script: String,
    pub run_script: String,
    pub test_cases: Vec<TestCase>,
}
