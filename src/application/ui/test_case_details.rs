use crate::application::common::*;
use crate::application::ui::ProblemScreenLayout;
use crate::application::ui::UIElement;
use crate::application::View;
use tui::widgets::Paragraph;
use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Style, Color},
    text::{Span, Spans},
    Frame,
};

#[memoize::memoize]
fn pretty_bar(curr: usize, max: usize) -> Spans<'static> {
    let line: Vec<Span<'static>> = (0..max)
        .map(|idx| if idx == curr { Span::styled(" ⬤ ", Style::default().fg(Color::Blue)) } else { Span::from(" ◯ ") } )
        .collect();
    Spans::from(line)
}

pub struct TestCaseDetails {
    test_case: TestCaseStatus,
    id: usize,
    total: usize
}

impl UIElement for TestCaseDetails {
    type ExpectedLayout = ProblemScreenLayout;

    fn setup(view: &View) -> Self {
        let (id, test_case) = view.details_for_selected_test_case();
        let total = view.get_n_problems();
        Self { id, test_case, total }
    }

    fn render<B: Backend>(self, frame: &mut Frame<B>, layout: &ProblemScreenLayout) {
        let mut lines = vec![
            Spans::from(format!("View of test case {} / {}:", self.id, self.total)), 
            Spans::from("")
        ];
        let mut details = self.test_case.into_detailed();
        lines.append(&mut details);
        
        let widget = Paragraph::new(lines);
        frame.render_widget(widget, layout.data);
        frame.render_widget(Paragraph::new(pretty_bar(self.id, self.total)).alignment(Alignment::Right), layout.data);
    }
}
