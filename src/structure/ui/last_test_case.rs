use crate::structure::ui::ProblemScreenLayout;
use tui::widgets::Paragraph;
use crate::structure::ui::UIElement;
use tui::{
    backend::{Backend},
    style::{Modifier, Style},
    text::{Span, Spans},
    Frame,
};
use crate::structure::common::*;
use crate::structure::View;

pub struct LastTestCaseView {
    compilation_fail: bool,
    expected_stdout: String,
    actual_stdout: Option<String>,
}

impl UIElement for LastTestCaseView {
    type ExpectedLayout = ProblemScreenLayout;

    fn setup(view: &View) -> Self {
        let compilation_fail = true;
        let expected_stdout = "fasdf".into();
        let actual_stdout = None;
        Self { compilation_fail, expected_stdout, actual_stdout }
    }

    fn render<B: Backend> (self, frame: &mut Frame<B>, layout: &ProblemScreenLayout) {
        if self.compilation_fail {
            let message = Paragraph::new(
                Span::styled("Compilation failed!", Style::default().fg(tui::style::Color::Red).add_modifier(Modifier::BOLD))
            );
            frame.render_widget(message, layout.data);
        } else {
            frame.render_widget(Paragraph::new("Hi there :))"), layout.data);
        }
    }
}

