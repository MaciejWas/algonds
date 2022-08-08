use crate::structure::controller::EventResult;
use crate::structure::view::Menu;
use crossterm::event::Event;
use std::iter::zip;
use tui::widgets::Paragraph;
use tui::{backend::Backend, Terminal};

use crate::structure::AppState;

impl AppState {
    pub fn render<B: Backend>(&self, term: &mut Terminal<B>) {
        match &self.view.curr_menu() {
            Menu::Solve => self.render_problem(term),
            Menu::Update => self.render_update(term),
            Menu::Select => self.render_select(term),
            Menu::Help => self.render_help(term),
        }
    }

    fn render_help<B: Backend>(&self, term: &mut Terminal<B>) {
        let term_size = term.size().unwrap();
        let help_text = self.view.get_help();
        let outer_window = problem_menu_utils::add_margins(term_size);
        let inner_window = problem_menu_utils::add_margins(outer_window);

        term.draw(|frame| {
            frame.render_widget(self.view.block_with_title("Help".to_string()), outer_window);
            frame.render_widget(help_text, inner_window);
            if inner_window.height < 14 {
                frame.render_widget(Paragraph::new("Window is too small to show all help!".to_string()), select_menu_utils::get_term_footnote(term_size))
            }
        })
        .unwrap();
    }

    fn render_problem<B: Backend>(&self, term: &mut Terminal<B>) {
        let term_size = term.size().unwrap();
        let layout = problem_menu_utils::layout_for_problem(term_size);

        let current_problem = self.view.detailed_problem();
            
        term.draw(|frame| {
            frame.render_widget(
                self.view.block_with_title("Solving".to_string()),
                layout.window,
            );
            frame.render_widget(current_problem.0, layout.problem_name);
            frame.render_widget(current_problem.1, layout.problem_statement);
            frame.render_widget(current_problem.2, layout.problem_example);
            frame.render_widget(self.view.additional_data(), layout.last_run_data);
        })
        .unwrap();
    }

    fn render_update<B: Backend>(&self, term: &mut Terminal<B>) {}

    fn render_select<B: Backend>(&self, term: &mut Terminal<B>) {
        let term_size = term.size().unwrap();
        let layout = select_menu_utils::layout_for_select(term_size);
        let row_n = layout.rows.len();

        let problems = self.view.get_problems_to_select(row_n);
        let (d_name, d_descr, d_example) = self.view.detailed_problem();

        term.draw(|frame| {
            frame.render_widget(
                self.view
                    .block_with_title("Available Challenges".to_string()),
                layout.rows_box,
            );
            frame.render_widget(
                self.view.block_with_title("Selected".to_string()),
                layout.problem_box,
            );

            for (problem, row) in zip(problems, layout.rows) {
                frame.render_widget(problem, row);
            }
            frame.render_widget(d_name, layout.problem_paragraphs.0);
            frame.render_widget(d_descr, layout.problem_paragraphs.1);
            frame.render_widget(d_example, layout.problem_paragraphs.2);

            frame.render_widget(
                Paragraph::new("Press h for help.".clone()),
                select_menu_utils::get_term_footnote(term_size),
            );
        })
        .unwrap();
    }

    pub fn update(&mut self) {}

    pub fn react_to_event(&mut self, event: Event) -> EventResult {
        self.controller.react_to_event(event)
    }
}

mod select_menu_utils {
    use tui::layout::{Constraint, Direction, Layout, Rect};

    pub fn get_term_footnote(term: Rect) -> Rect {
        let footnote_splitter = Layout::default()
            .constraints(vec![Constraint::Max(term.height), Constraint::Length(1)]);
        footnote_splitter.split(term)[1]
    }

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
        let div3 = (term_size.height - 2) as usize / 3;
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
            problem_paragraphs,
        }
    }

    pub struct SelectMenuLayout {
        pub rows_box: Rect,
        pub rows: Vec<Rect>,
        pub problem_box: Rect,
        pub problem_paragraphs: (Rect, Rect, Rect),
    }
}

mod problem_menu_utils {
    use tui::layout::Constraint;
    use tui::layout::Direction;
    use tui::layout::Layout;
    use tui::layout::Margin;
    use tui::layout::Rect;

    // const split_10_40_40_10: Vec<Constraint> = vec![
    //     Constraint::Percentage(10),
    //     Constraint::Percentage(40),
    //     Constraint::Percentage(40),
    //     Constraint::Percentage(10),
    // ];

    pub struct ProblemLayout {
        pub window: Rect,
        pub problem_name: Rect,
        pub problem_statement: Rect,
        pub problem_example: Rect,
        pub last_run_data: Rect,
    }

    pub fn add_margins(term_size: Rect) -> Rect {
        term_size.inner(&Margin {
            vertical: 3,
            horizontal: 3,
        })
    }

    pub fn layout_for_problem(term_size: Rect) -> ProblemLayout {
        let window = term_size; // add_margins(term_size);

        let split_10_40_40_10: Vec<Constraint> = vec![
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
            Constraint::Percentage(10),
        ];

        let vertical_splitter = Layout::default()
            .constraints(split_10_40_40_10)
            .margin(6)
            .direction(Direction::Vertical);

        let paragraphs = vertical_splitter.split(window);
        ProblemLayout {
            window: window,
            problem_name: paragraphs[0],
            problem_statement: paragraphs[1],
            problem_example: paragraphs[2],
            last_run_data: paragraphs[3],
        }
    }
}

mod popup_utils {
    use tui::layout::Layout;
    use tui::layout::Constraint;
    use tui::layout::Rect;

    // const vertical_split_data: Vec<Constraint> = vec![
    //         Constraint::Percentage(25),
    //         Constraint::Percentage(50),
    //         Constraint::Percentage(25),
    //     ];

    pub fn popup_box(term_size: Rect) -> Rect {
        let vertical_split_data = vec![
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ];

        let horizontal_split_data = vec![
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ];

        let vertical_splitter = Layout::default().constraints(vertical_split_data).direction(tui::layout::Direction::Vertical);
        let horizontal_splitter = Layout::default().constraints(horizontal_split_data).direction(tui::layout::Direction::Horizontal);
        
        horizontal_splitter.split(vertical_splitter.split(term_size)[1])[1]
    }
}