use crate::application::View;
use tui::{backend::Backend, Frame};

mod available_problems;
mod commands;
mod full_problem;
mod help;
mod last_test_case;
mod layouts;
mod problem_view;
mod test_case_table;

pub use available_problems::AvailableProblems;
pub use commands::CommandsView;
pub use full_problem::FullProblem;
pub use help::Help;
pub use last_test_case::LastTestCaseView;
pub use layouts::*;
pub use problem_view::ProblemView;
pub use test_case_table::TestCaseTable;

pub trait UIElement {
    type ExpectedLayout;
    fn setup(view: &View) -> Self;
    fn render<B: Backend>(self, frame: &mut Frame<B>, layout: &Self::ExpectedLayout);
}
