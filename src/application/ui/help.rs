use crate::application::ui::HelpScreenLayout;
use crate::application::ui::UIElement;
use crate::application::View;
use tui::widgets::Paragraph;
use tui::widgets::Wrap;
use tui::{
    backend::Backend,
    text::{Span, Spans},
    Frame,
};


#[memoize::memoize]
fn create_general_help(_unit: ()) -> Paragraph<'static> {
    let spans = vec![
        Spans::from(bold("General help")),
        Spans::from("  h - open help"),
        Spans::from("  q - quit current menu"),
        Spans::from("  ctrl + c - exit application"),
        Spans::from(""),
    ];
    Paragraph::new(spans).wrap(Wrap { trim: false })
}

#[memoize::memoize]
fn create_select_help(_unit: ()) -> Paragraph<'static> {
    let spans = vec![
        Spans::from(bold("When selecting problem")),
        Spans::from("  up/down (k/j) - open help"),
        Spans::from("  enter - select problem"),
        Spans::from(""),
    ];
    Paragraph::new(spans).wrap(Wrap { trim: false })
}

#[memoize::memoize]
fn create_solve_help(_unit: ()) -> Paragraph<'static> {
    let spans = vec![
        Spans::from(bold("When solving problem")),
        Spans::from("  c - edit compile script"),
        Spans::from("  r - edit run script"),
        Spans::from("  enter - run all test cases"),
        Spans::from("  backspace - cancel running test cases"),
        Spans::from("  t - see status of test cases"),
        Spans::from("  s - see run/compile scripts"),
        Spans::from("  d - see test cases details"),
        Spans::from("  p - see performance"),
        Spans::from(""),
    ];
    Paragraph::new(spans).wrap(Wrap { trim: false })
}



fn bold<'a, T: Into<String>>(text: T) -> Span<'a> {
    Span::styled(
        text.into(),
        tui::style::Style::default().add_modifier(tui::style::Modifier::BOLD),
    )
}

pub struct Help {
    general_help: Paragraph<'static>,
    select_help: Paragraph<'static>,
    solve_help: Paragraph<'static>,
}

impl UIElement for Help {
    type ExpectedLayout = HelpScreenLayout;

    fn setup(_view: &View) -> Self {
        let general_help = create_general_help(());
        let select_help = create_select_help(());
        let solve_help = create_solve_help(());
        Self {
            general_help,
            select_help,
            solve_help,
        }
    }

    fn render<B: Backend>(self, frame: &mut Frame<B>, layout: &HelpScreenLayout) {
        frame.render_widget(self.general_help, layout.general_help);
        frame.render_widget(self.select_help, layout.select_help);
        frame.render_widget(self.solve_help, layout.solve_help);
    }
}
