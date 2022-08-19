use crate::structure::ui::HelpScreenLayout;
use tui::widgets::Wrap;
use tui::widgets::Paragraph;
use crate::structure::ui::UIElement;
use std::rc::Rc;
use tui::layout::Rect;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
use crate::structure::common::*;
use crate::structure::View;
use crate::structure::ui::MenuLayout;



fn bold<'a, T: Into<String>>(text: T) -> Span<'a> {
    Span::styled(
        text.into(),
        tui::style::Style::default().add_modifier(tui::style::Modifier::BOLD),
    )
}

fn text<'a>(t: String) -> Span<'a> {
    Span::from(t)
}

pub struct Help<'a> {
    general_help: Paragraph<'a>,
    select_help: Paragraph<'a>,
    solve_help: Paragraph<'a>,
}

impl<'a> Help<'a> {
    fn create_general_help() -> Paragraph<'a> {
        let spans = vec![
            Spans::from(bold("General help")),
            Spans::from("  h - open help"),
            Spans::from("  q - quit current menu"),
            Spans::from("  ctrl + c - exit application"),
            Spans::from(""),
        ];
        Paragraph::new(spans).wrap(Wrap { trim: false })
    }

    fn create_select_help() -> Paragraph<'a> {
        let spans = vec![
            Spans::from(bold("Select menu")),
            Spans::from("  up/down (k/j) - open help"),
            Spans::from("  enter - select problem"),
            Spans::from(""),
        ];
        Paragraph::new(spans).wrap(Wrap { trim: false })
    }

    fn create_solve_help() -> Paragraph<'a> {
        let spans = vec![
            Spans::from(bold("Solve menu")),
            Spans::from("  c - edit compile script"),
            Spans::from("  r - edit run script"),
            Spans::from("  enter - run all test cases"),
            Spans::from("  d - see last run details"),
            Spans::from(""),
        ];
        Paragraph::new(spans).wrap(Wrap { trim: false })
    }
}

impl<'a> UIElement for Help<'a> {
    type ExpectedLayout = HelpScreenLayout;

    fn setup(view: &View) -> Self {
        let general_help = Self::create_general_help();
        let select_help = Self::create_select_help();
        let solve_help = Self::create_solve_help();
        Self { general_help, select_help, solve_help }
    }

    fn render<B: Backend> (self, frame: &mut Frame<B>, layout: &HelpScreenLayout) {
        frame.render_widget(self.general_help, layout.general_help);
        frame.render_widget(self.select_help, layout.select_help);
        frame.render_widget(self.solve_help, layout.solve_help);
    }
}

