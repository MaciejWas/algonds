use crate::application::common::*;
use crate::application::ui::ProblemScreenLayout;
use crate::application::ui::UIElement;
use crate::application::View;
use tui::widgets::Paragraph;
use tui::{
    backend::Backend,
    style::{Modifier, Style},
    text::{Span, Spans},
    Frame,
};

pub struct LastTestCaseView {
    last_failed: Option<(usize, TestCaseStatus)>,
}

impl UIElement for LastTestCaseView {
    type ExpectedLayout = ProblemScreenLayout;

    fn setup(view: &View) -> Self {
        let last_failed = view.get_last_failed();
        Self { last_failed }
    }

    fn render<B: Backend>(self, frame: &mut Frame<B>, layout: &ProblemScreenLayout) {
        let widget = match self.last_failed {
            Some((n, TestCaseStatus::Err { err_msg })) => Paragraph::new(Spans::from(vec![
                Span::from(format!("Error processing test case {}:", n)),
                Span::styled(err_msg, Style::default().fg(tui::style::Color::Red)),
            ])),
            _ => Paragraph::new(Span::styled(
                "Good job! All tests passed (or just weren't run)",
                Style::default()
                    .fg(tui::style::Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
        };

        frame.render_widget(widget, layout.data);
    }
}
