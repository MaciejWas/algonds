use crate::structure::ui::ProblemLayout;
use crate::structure::ui::SelectScreenLayout;
use tui::widgets::Wrap;
use tui::widgets::Paragraph;
use crate::structure::ui::UIElement;
use std::rc::Rc;
use tui::layout::Rect;
use tui::{
    backend::Backend,
    text::{Span, Spans},
    widgets::{Block, Borders},
    Frame
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
        let fst_example = problem.examples.get(0).unwrap();
        let title = Paragraph::new(Spans::from(bold(problem.problem_name.clone())))
            .alignment(tui::layout::Alignment::Center);
        let statement = Paragraph::new(Spans::from(text(problem.problem_statement.clone())))
            .wrap( Wrap { trim: false });
        let example = example(fst_example)
            .wrap(Wrap { trim: false });

        Self { title, statement, example }
    }

    fn render<B: Backend> (self, frame: &mut Frame<B>, layout: &ProblemLayout) {
        let ProblemLayout { title, statement, example } = layout.clone();
        frame.render_widget(self.title, title);
        frame.render_widget(self.statement, statement);
        frame.render_widget(self.example, example);
    }
}

