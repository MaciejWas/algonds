use crate::structure::common::*;
use crate::structure::ui::ProblemScreenLayout;
use crate::structure::ui::UIElement;
use crate::structure::View;
use tui::widgets::Paragraph;
use tui::{
    backend::Backend,
    layout::Constraint,
    text::{Span, Spans},
    widgets::{Cell, Row, Table},
    Frame,
};

pub struct TestCaseTable {
    test_cases: Vec<TestCaseStatus>,
}

impl UIElement for TestCaseTable {
    type ExpectedLayout = ProblemScreenLayout;

    fn setup(view: &View) -> Self {
        let test_cases = view.get_test_cases();
        Self { test_cases }
    }

    fn render<B: Backend>(self, frame: &mut Frame<B>, layout: &ProblemScreenLayout) {
        let display_statuses = self
            .test_cases
            .into_iter()
            .map(TestCaseStatus::into_span)
            .map(Cell::from)
            .enumerate();

        let mut row_1 = Vec::new();
        let mut row_2 = Vec::new();
        let mut row_3 = Vec::new();

        for (i, status) in display_statuses {
            match i % 3 {
                0 => row_1.push(status),
                1 => row_2.push(status),
                2 => row_3.push(status),
                _ => {}
            }
        }

        let constraints = vec![Constraint::Length(15); 12];

        let test_case_data = Table::new([Row::new(row_1), Row::new(row_2), Row::new(row_3)])
            .column_spacing(3)
            .widths(&constraints);

        frame.render_widget(test_case_data, layout.data);
    }
}
