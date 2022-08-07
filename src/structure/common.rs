use crate::structure::Difficulty::Easy;
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
    pub problem_example: String,
    pub tags: Vec<String>,
    pub difficulty: Difficulty,
}

impl Default for Problem {
    fn default() -> Self {
        Problem {problem_name: "Reverse Linked Binary HashTable".into(), problem_statement: "statementstatementstatementstatemenementstatementstatementstatementstatementstementstatementstatementstatementstatementstementstatementstatementstatementstatementstementstatementstatementstatementstatementsttstatementstatementstatementstatementstatementstatementstatementstatementstatementstatementstatementstatementstatement".into(), 
    problem_example: "exampleexampleexampleexampleexampleempleexampleexampleempleexampleexampleempleexampleexampleexampleexampleexampleexampleexampleexampleexampleexample".into(), tags: vec!["tag1".into(), "tag2fsadf".into(), "tag1".into(), "tag2fsadf".into(), "tag1".into(), "tag2fsadf".into()], difficulty: Easy}
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Example {
    input: String,
    output: String,
}

pub enum ExampleStatus {
    Pass,
    Fail,
    Running,
    NotRun,
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
