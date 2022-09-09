use crate::application::ui::UIElement;
use crate::application::common::*;
use crate::application::ui::ProblemLayout;
use crate::application::View;
use tui::widgets::Paragraph;
use tui::widgets::Wrap;
use tui::{
    backend::Backend,
    text::{Span, Spans},
    Frame,
};

fn test_case<'a>(tc: &TestCase) -> Paragraph<'a> {
    Paragraph::new(vec![
        Spans::from(bold("Example:".to_string())),
        Spans::from("    input: \n\n".to_string() + &tc.input),
        Spans::from("    output: \n\n".to_string() + &tc.output),
    ])
}

fn bold<'a>(text: String) -> Span<'a> {
    Span::styled(
        text,
        tui::style::Style::default().add_modifier(tui::style::Modifier::BOLD),
    )
}

fn text<'a>(t: String) -> Span<'a> {
    Span::from(t)
}

pub struct ProblemView<'a> {
    pub title: Paragraph<'a>,
    pub statement: Paragraph<'a>,
    pub example: Paragraph<'a>,
}

impl<'a> UIElement for ProblemView<'a> {
    type ExpectedLayout = ProblemLayout;

    fn setup(view: &View) -> Self {
        let problem = view.current_problem();
        let fst_example = problem.test_cases.get(0).unwrap();
        let title = Paragraph::new(Spans::from(bold(problem.name.clone())))
            .alignment(tui::layout::Alignment::Center);
        
        let statement_with_line_breaks: Vec<Spans> = problem.statement.clone()
            .replace("\t", "    ")
            .split("\n")
            .map(|line| Spans::from(text(line.to_string())))
            .collect();

        let statement = Paragraph::new(statement_with_line_breaks)
            .wrap(Wrap { trim: false });
        let example = test_case(fst_example).wrap(Wrap { trim: false });

        Self {
            title,
            statement,
            example,
        }
    }

    fn render<B: Backend>(self, frame: &mut Frame<B>, layout: &ProblemLayout) {
        let ProblemLayout {
            title,
            statement,
            example,
        } = *layout;
        frame.render_widget(self.title, title);
        frame.render_widget(self.statement, statement);
        frame.render_widget(self.example, example);
    }
}
