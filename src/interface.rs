use crate::application::common::Menu;
use crate::application::controller::AfterEvent;
use crate::application::controller::Controller;
use crate::application::ui;
use crate::application::ui::HelpScreenLayout;
use crate::application::ui::ProblemMenuLayout;
use crate::application::ui::SelectScreenLayout;
use crate::application::ui::UIElement;
use crossterm::event::Event;
use tui::widgets::Paragraph;
use tui::{backend::Backend, Frame};

use crate::application::AppState;

impl AppState {
    pub fn render<B: Backend>(&mut self, term: &mut Frame<B>) {
        match &self.view.curr_menu() {
            Menu::Solve => self.render_problem(term),
            Menu::Select => self.render_select(term),
            Menu::Help => self.render_help(term),
        }
    }

    fn render_help<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let term_size = frame.size();
        let layout = HelpScreenLayout::from(term_size);
        let help = ui::Help::setup(&self.view);
        help.render(frame, &layout);
    }

    fn render_problem<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let term_size = frame.size();
        let layout = ProblemMenuLayout::from(term_size);
        let problem = ui::FullProblem::setup(&self.view);
        problem.render(frame, &layout);
    }

    fn render_select<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let term_size = frame.size();
        let layout = SelectScreenLayout::from(term_size);
        let available_problems = ui::AvailableProblems::setup(&self.view);
        let problem_view = ui::ProblemView::setup(&self.view);

        available_problems.render(frame, &layout);
        problem_view.render(frame, &layout.problem_preview);
    }

    pub fn react_to_event(&mut self, event: Event) -> AfterEvent {
        self.controller.react_to_event(event)
    }

    pub fn react_to_code_runner(&mut self) -> AfterEvent {
        match self.view.check_for_changes() {
            true => AfterEvent::DoRefresh,
            false => AfterEvent::NoRefresh,
        }
    }
}
