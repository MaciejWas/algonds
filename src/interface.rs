use tui::layout::Rect;
use tui::widgets::Widget;
use tui::widgets::Block;
use crate::structure::controller::EventResult;
use crate::structure::view::Menu;
use crossterm::event::Event;
use std::iter::zip;
use tui::{
    backend::Backend,
    Terminal,
    widgets::Borders
};

use crate::structure::AppState;

impl AppState {
    pub fn render<B: Backend>(&self, term: &mut Terminal<B>) {
        match &self.view.curr_menu() {
            Menu::Solve => {
                self.render_problem(term);
            }
            Menu::Update => {
                self.render_update(term);
            }
            Menu::Select => {
                self.render_select(term);
            }
        }
    }

    fn render_problem<B: Backend>(&self, term: &mut Terminal<B>) {}

    fn render_update<B: Backend>(&self, term: &mut Terminal<B>) {}

    fn render_select<B: Backend>(&self, term: &mut Terminal<B>) {
        let term_size = term.size().unwrap();
        let layout = select_menu_utils::layout_for_select(term_size);
        let row_n = layout.rows.len();

        let problems = self.view.get_problems_to_select(row_n);
        let (d_name, d_descr, d_example) = self.view.detailed_problem();

        term.draw(|frame| {
            frame.render_widget(Self::borders("Available Challenges"), layout.rows_box);
            frame.render_widget(Self::borders("Selected"), layout.problem_box);

            for (problem, row) in zip(problems, layout.rows) {
                frame.render_widget(problem, row);
            }
            frame.render_widget(d_name, layout.problem_paragraphs.0);
            frame.render_widget(d_descr, layout.problem_paragraphs.1);
            frame.render_widget(d_example, layout.problem_paragraphs.2);
        })
        .unwrap();
    }

    fn borders<'a>(title: &'a str) -> Block<'a> {
        Block::default().borders(Borders::ALL).title(title)
    } 

    pub fn update(&mut self) {}

    pub fn react_to_event(&mut self, event: Event) -> EventResult {
        self.controller.react_to_event(event)
    }
}


mod select_menu_utils {
    use tui::{
        layout::{Constraint, Direction, Layout, Rect},
    };

    fn do_horizontal_split_for_select_menu(term_size: Rect) -> (Rect, Rect) {
        let horizontal_split_data = vec![Constraint::Percentage(60), Constraint::Percentage(40)];
        let horizontal_splitter = Layout::default()
            .constraints(horizontal_split_data)
            .margin(1)
            .direction(Direction::Horizontal);
        let windows = horizontal_splitter.split(term_size);
        (windows[0], windows[1])
    }

    fn do_row_split_for_select_menu(term_size: Rect) -> Vec<Rect> {
        let div3 = term_size.height as usize / 3;
        let row_split_data = vec![Constraint::Length(3); div3];
        let row_splitter = Layout::default()
            .constraints(row_split_data)
            .margin(2)
            .direction(Direction::Vertical);
        let rows = row_splitter.split(term_size);
        rows
    }

    fn do_vertical_split_for_select_menu(term_size: Rect) -> (Rect, Rect, Rect) {
        let vertical_split_data = vec![
            Constraint::Percentage(10),
            Constraint::Percentage(50),
            Constraint::Percentage(40),
        ];
        let vertical_splitter = Layout::default()
            .constraints(vertical_split_data)
            .margin(1)
            .direction(Direction::Vertical);
        let paragraphs = vertical_splitter.split(term_size);
        (paragraphs[0], paragraphs[1], paragraphs[2])
    }

    pub fn layout_for_select(term_size: Rect) -> SelectMenuLayout {
        let (rows_box, problem_box) = do_horizontal_split_for_select_menu(term_size);
        let rows = do_row_split_for_select_menu(rows_box);
        let problem_paragraphs = do_vertical_split_for_select_menu(problem_box);
        SelectMenuLayout {
            rows_box,
            rows,
            problem_box,
            problem_paragraphs
        }
    }

    pub struct SelectMenuLayout {
        pub rows_box: Rect,
        pub rows: Vec<Rect>,
        pub problem_box: Rect,
        pub problem_paragraphs: (Rect, Rect, Rect)
    }
}