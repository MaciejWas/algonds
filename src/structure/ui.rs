use tui::{
    backend::Backend,
    Frame,
};
use crate::structure::View;

mod available_problems;
mod problem_view;
mod layouts;
mod help;
mod test_case_table;
mod full_problem;
mod commands;
mod last_test_case;

pub use commands::CommandsView;
pub use available_problems::AvailableProblems;
pub use problem_view::ProblemView;
pub use help::Help;
pub use test_case_table::TestCaseTable;
pub use layouts::*;
pub use full_problem::FullProblem;
pub use last_test_case::LastTestCaseView;

pub trait UIElement {
    type ExpectedLayout;
    fn setup(view: &View) -> Self;
    fn render<B: Backend>(self, frame: &mut Frame<B>, layout: &Self::ExpectedLayout);
}
