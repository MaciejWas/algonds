use crate::structure::ui::SelectScreenLayout;
use crate::structure::ui::ProblemScreenLayout;
use crate::structure::ui::HelpScreenLayout;
use crate::structure::ui;
use crate::structure::ui::MenuLayout;
use crate::structure::common::Menu;
use crate::structure::controller::AfterEvent;
use crate::structure::ui::UIElement;
use crossterm::event::Event;
use std::iter::zip;
use tui::{
    backend::{Backend},
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{Paragraph, Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

use crate::structure::AppState;

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
        let layout = ProblemScreenLayout::from(term_size);
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

    pub fn update(&mut self) {}

    pub fn react_to_event(&mut self, event: Event) -> AfterEvent {
        self.controller.react_to_event(event)
    }

    pub fn react_to_code_runner(&mut self) -> AfterEvent {
        AfterEvent::NoRefresh // todo: Impl;
    }
}