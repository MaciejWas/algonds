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

pub struct CommandsView {
    compile_command: String,
    run_command: String,
    selected: Option<InputField>,
}

impl UIElement for CommandsView {
    type ExpectedLayout = ProblemScreenLayout;

    fn setup(view: &View) -> Self {
        let compile_command = view.compile_command_view();
        let run_command = view.run_command_view();
        let selected = view.curr_field();
        Self {
            selected,
            compile_command,
            run_command,
        }
    }

    fn render<B: Backend>(self, frame: &mut Frame<B>, layout: &ProblemScreenLayout) {
        let bold_style = Style::default().add_modifier(Modifier::BOLD);

        let (compile_style, run_style) = match &self.selected {
            Some(InputField::RunCommand) => (Style::default(), bold_style),
            Some(InputField::CompileCommand) => (bold_style, Style::default()),
            None => (Style::default(), Style::default()),
        };

        let commands = Paragraph::new(vec![
            Spans::from(Span::styled(
                "[C]ompile command: ".to_string() + &self.compile_command,
                compile_style,
            )),
            Spans::from(Span::styled(
                "[R]un Command: ".to_string() + &self.run_command,
                run_style,
            )),
        ]);

        frame.render_widget(commands, layout.data);
    }
}
