use crate::application::common::ProblemDataKind;
use crate::application::ui::*;
use crate::application::View;
use tui::backend::Backend;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::Frame;

#[memoize::memoize]
fn make_problem_block(unit: ()) -> Block<'static> {
    Block::default().borders(Borders::ALL).title("Solving")
}

#[memoize::memoize]
fn make_problem_data_border(tab: u8) -> Block<'static> {
    let selected_style = Style::default()
        .add_modifier(tui::style::Modifier::BOLD)
        .fg(tui::style::Color::Green);

    let title = match tab {
        0 => Spans::from(vec![
            Span::styled(" [T]est cases", selected_style),
            Span::from("  |  "),
            Span::from("[S]etup:"),
            Span::from("  |  "),
            Span::from("[D]etails "),
        ]),
        1 => Spans::from(vec![
            Span::from(" [T]est cases"),
            Span::from("  |  "),
            Span::styled("[S]etup:", selected_style),
            Span::from("  |  "),
            Span::from("[D]etails "),
        ]),
        2 => Spans::from(vec![
            Span::from(" [T]est cases"),
            Span::from("  |  "),
            Span::from("[S]etup:"),
            Span::from("  |  "),
            Span::styled("[D]etails ", selected_style),
        ]),
        _ => unreachable!(),
    };
    Block::default().borders(Borders::ALL).title(title)
}

enum ProblemData {
    TestCases(TestCaseTable),
    Details(TestCaseDetails),
    Commands(CommandsView),
}

impl Into<u8> for &ProblemData {
    fn into(self) -> u8 {
        match self {
            ProblemData::TestCases(_) => 0,
            ProblemData::Commands(_) => 1,
            ProblemData::Details(_) => 2,
        }
    }
}

impl UIElement for ProblemData {
    type ExpectedLayout = ProblemScreenLayout;

    fn setup(view: &View) -> Self {
        let to_show = view.curr_data();
        match to_show {
            ProblemDataKind::TestCases => Self::TestCases(TestCaseTable::setup(&view)),
            ProblemDataKind::Commands => Self::Commands(CommandsView::setup(&view)),
            ProblemDataKind::Details => Self::Details(TestCaseDetails::setup(&view)),
        }
    }

    fn render<B: tui::backend::Backend>(self, frame: &mut Frame<B>, layout: &ProblemScreenLayout) {
        match self {
            Self::TestCases(widget) => widget.render(frame, layout),
            Self::Commands(widget) => widget.render(frame, layout),
            Self::Details(widget) => widget.render(frame, layout),
        }
    }
}

pub struct FullProblem<'a> {
    problem_data: ProblemView<'a>,
    run_data: ProblemData,
}

impl<'a> UIElement for FullProblem<'a> {
    type ExpectedLayout = ProblemScreenLayout;
    fn setup(view: &View) -> Self {
        let problem_data = ProblemView::setup(view);
        let run_data = ProblemData::setup(view);
        Self {
            problem_data,
            run_data,
        }
    }
    fn render<B>(self, frame: &mut Frame<B>, layout: &ProblemScreenLayout)
    where
        B: Backend,
    {
        let problem_view_border = make_problem_block(());
        let problem_data_border = make_problem_data_border((&self.run_data).into());

        frame.render_widget(problem_view_border, layout.problem_window);
        frame.render_widget(problem_data_border, layout.data_window);

        self.problem_data.render(frame, &layout.problem);
        self.run_data.render(frame, &layout)
    }
}
