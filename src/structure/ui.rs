use tui::{
    backend::Backend,
    Frame,
};
use crate::structure::View;

mod available_problems;
mod problem_view;
mod layouts;
mod help;
mod problem_data;
mod full_problem;

pub use available_problems::AvailableProblems;
pub use problem_view::ProblemView;
pub use help::Help;
pub use problem_data::ProblemData;
pub use layouts::*;
pub use full_problem::FullProblem;

pub trait UIElement {
    type ExpectedLayout;
    fn setup(view: &View) -> Self;
    fn render<B: Backend>(self, frame: &mut Frame<B>, layout: &Self::ExpectedLayout);
}