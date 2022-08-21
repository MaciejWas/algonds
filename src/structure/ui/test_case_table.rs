use crate::structure::ui::ProblemScreenLayout;
use crate::structure::ui::SelectScreenLayout;
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
    widgets::{Block, Row, Borders, Table, Cell},
    Frame, Terminal,
};
use crate::structure::common::*;
use crate::structure::View;

fn example<'a>(exmp: &Example) -> Paragraph<'a> {
    Paragraph::new(vec![
        Spans::from(bold("Example:".to_string())),
        Spans::from("    input: \n\n".to_string() + &exmp.input),
        Spans::from("    output: \n\n".to_string() + &exmp.output),
    ])
}

fn bold<'a>(text: impl Into<String>) -> Span<'a> {
    Span::styled(
        text.into(),
        tui::style::Style::default().add_modifier(tui::style::Modifier::BOLD),
    )
}

fn text<'a>(t: String) -> Span<'a> {
    Span::from(t)
}

pub struct TestCaseTable<'a> {
    pub x: &'a ()
}

impl<'a> UIElement for TestCaseTable<'a> {
    type ExpectedLayout = ProblemScreenLayout;

    fn setup(view: &View) -> Self {
        Self { x: &() }
    }

    fn render<B: Backend> (self, frame: &mut Frame<B>, layout: &ProblemScreenLayout) {
        let rows = vec![
            Row::new(vec![Cell::from("  🗹 Pass"), Cell::from("  ⚠ Cancelled"), Cell::from("  ‼ Error")]),
            Row::new(vec![Cell::from("  🗷 Fail"), Cell::from("  🯄 NotRun")]),
            Row::new(vec![Cell::from("  ⌛ Running"), ])
        ];

        let constraints = vec![Constraint::Length(10); 12];

        let test_case_data = Table::new(rows)
            .column_spacing(3)
            .widths(&constraints);

        frame.render_widget(test_case_data, layout.data);
    }
}

