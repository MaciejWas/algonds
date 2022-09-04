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

pub struct TestCaseView {
    test_case: TestCaseStatus,
    id: usize
}

impl UIElement for TestCaseView {
    type ExpectedLayout = ProblemScreenLayout;

    fn setup(view: &View) -> Self {
        let (id, test_case) = view.details();
        Self { id, test_case }
    }

    fn render<B: Backend>(self, frame: &mut Frame<B>, layout: &ProblemScreenLayout) {
        let mut lines = vec![Spans::from(format!("View of test case {}:", self.id)), Spans::from("")];
        let mut details = self.test_case.into_detailed();
        lines.append(&mut details);
        
        let widget = Paragraph::new(lines);
        frame.render_widget(widget, layout.data);
    }
}
