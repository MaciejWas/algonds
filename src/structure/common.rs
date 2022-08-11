use tui::text::Spans;
use tui::widgets::Paragraph;
use serde::{Deserialize, Serialize};
use tui::style::Color;
use tui::style::Style;
use tui::text::Span;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

pub enum ExampleStatus {
    Pass,
    Fail,
    Running,
    NotRun,
}

impl Default for ExampleStatus {
    fn default() -> Self { Self::NotRun }
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


#[derive(Debug, Clone, Copy)]
pub enum InputField {
    CompileCommand,
    RunCommand,
}

pub enum AdditionalData {
    CompileCommand(String),
    RunCommand(String),
    RunningBar(u8),
    None,
}

impl<'a> Into<Paragraph<'a>> for AdditionalData {   
    fn into(self) -> Paragraph<'a> { 
        match self {
            Self::CompileCommand(command) => Paragraph::new(Spans::from("Edit compilation step: ".to_string() + &command + "▮")),
            Self::RunCommand(command) => Paragraph::new(Spans::from("Edit run step: ".to_string() + &command + "▮")),
            Self::RunningBar(u) => Paragraph::new(Spans::from("Running: ".to_string() + &u.to_string())),
            Self::None => Paragraph::new(Spans::from("r - edit run script, c - edit compilation script, enter - run tests")),
        }
    }
}