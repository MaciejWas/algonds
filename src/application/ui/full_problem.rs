use crate::application::common::ProblemDataKind;
use crate::application::ui::CommandsView;
use crate::application::ui::TestCaseDetails;
use crate::application::ui::ProblemScreenLayout;
use crate::application::ui::ProblemView;
use crate::application::ui::TestCaseTable;
use crate::application::View;
use crate::UIElement;
use tui::backend::Backend;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::Frame;

#[memoize::memoize]
fn make_title(tab: u8) -> Spans<'static> {
    let selected_style = Style::default()
        .add_modifier(tui::style::Modifier::BOLD)
        .fg(tui::style::Color::Green);

    match tab {
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
        _ => unreachable!()
    }
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
            ProblemData::Details(_) => 2
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
            ProblemDataKind::Details => Self::Details(TestCaseDetails::setup(&view))
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
        let title = make_title((&self.run_data).into());
        let problem_view_border = Block::default().borders(Borders::ALL).title("Solving");
        let problem_data_border = Block::default().borders(Borders::ALL).title(title);

        frame.render_widget(problem_view_border, layout.problem_window);
        frame.render_widget(problem_data_border, layout.data_window);

        self.problem_data.render(frame, &layout.problem);
        self.run_data.render(frame, &layout)
    }
}
