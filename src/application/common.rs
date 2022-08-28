// use crate::application::AdditionalData::*;
use serde::{Deserialize, Serialize};
use tui::style::Color;
use tui::style::Style;
use tui::text::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    Select,
    Solve,
    Help,
}

#[derive(Copy, Clone)]
pub enum ProblemDataKind {
    TestCases,
    Commands,
    LastFailedExample,
}

impl Default for ProblemDataKind {
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
    pub problem_name: String,
    pub problem_statement: String,
    pub tags: Vec<String>,
    pub examples: Vec<Example>,
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Example {
    pub input: String,
    pub output: String,
}

impl TestCaseStatus {
    pub fn into_span<'a>(self) -> Span<'a> {
        let text = match &self {
            Self::Pass { .. } => "🗹 Pass",
            Self::Fail { .. } => "🗷 Fail",
            Self::Running => "⌛ Running",
            Self::Cancelled => "⚠ Cancelled",
            Self::NotRun => "🯄 NotRun",
            Self::Err { .. } => "‼ Error",
        };
        let style = match &self {
            Self::Pass { .. } => Style::default().fg(Color::Green),
            Self::Fail { .. } | Self::Err { .. } => Style::default().fg(Color::Red),
            Self::Cancelled | Self::Running => Style::default().fg(Color::Yellow),
            Self::NotRun => Style::default().fg(Color::Gray),
        };
        Span::styled(text, style)
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

#[derive(Debug, Clone)]
pub enum TestCaseStatus {
    Pass { actual: String },
    Fail { expected: String, actual: String },
    Err { err_msg: String },
    Cancelled,
    Running,
    NotRun,
}

impl TestCaseStatus {
    pub fn is_err(&self) -> bool {
        match self {
            Self::Err { .. } => true,
            _ => false,
        }
    }
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
    pub examples: Vec<Example>,
}
