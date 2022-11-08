use crate::application::common::ProblemDataTab;
use crate::application::ui::*;
use crate::application::View;
use tui::backend::Backend;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::Frame;
use tui::widgets::Paragraph;

enum ProblemTabs {
    TestCaseTab(TestCaseTable),
    DetailsTab(TestCaseDetails),
    CommandsTab(CommandsView),
    PerformanceTab(PerformanceChart)
}

impl ProblemTabs {
    fn code(&self) -> u8 {
        match self {
            ProblemTabs::TestCaseTab(_) => 0,
            ProblemTabs::CommandsTab(_) => 1,
            ProblemTabs::DetailsTab(_) => 2,
            ProblemTabs::PerformanceTab(_) => 3,
        }
    }
}

impl UIElement for ProblemTabs {
    type ExpectedLayout = ProblemMenuLayout;

    fn setup(view: &View) -> Self {
        let to_show = view.curr_data();
        match to_show {
            ProblemDataTab::TestCases => Self::TestCaseTab(TestCaseTable::setup(view)),
            ProblemDataTab::Commands => Self::CommandsTab(CommandsView::setup(view)),
            ProblemDataTab::Details => Self::DetailsTab(TestCaseDetails::setup(view)),
            ProblemDataTab::Performance => Self::PerformanceTab(PerformanceChart::setup(view))
        }
    }

    fn render<B: tui::backend::Backend>(self, frame: &mut Frame<B>, layout: &ProblemMenuLayout) {
        match self {
            Self::TestCaseTab(widget) => widget.render(frame, layout),
            Self::CommandsTab(widget) => widget.render(frame, layout),
            Self::DetailsTab(widget) => widget.render(frame, layout),
            Self::PerformanceTab(widget) => widget.render(frame, layout)
        }
    }
}

pub struct FullProblem<'a> {
    problem_data: ProblemView<'a>,
    run_data: ProblemTabs,
}

impl<'a> UIElement for FullProblem<'a> {
    type ExpectedLayout = ProblemMenuLayout;
    fn setup(view: &View) -> Self {
        let problem_data = ProblemView::setup(view);
        let run_data = ProblemTabs::setup(view);
        Self {
            problem_data,
            run_data,
        }
    }

    fn render<B>(self, frame: &mut Frame<B>, layout: &ProblemMenuLayout)
    where
        B: Backend,
    {
        let problem_view_border = make_problem_border(());
        let problem_data_border = make_problem_data_border(self.run_data.code());

        frame.render_widget(problem_view_border, layout.problem_window);
        frame.render_widget(problem_data_border, layout.problem_tabs_window);
        frame.render_widget( problem_menu_help(()), layout.footnote);

        self.problem_data.render(frame, &layout.problem);
        self.run_data.render(frame, layout);
    }
}


#[memoize::memoize]
fn problem_menu_help(_unit: ()) -> Paragraph<'static> {
    return Paragraph::new("q - quit,   h - help,   press keys in [b]rackets to use menu items,  use arrows to navigate problem details")
        .alignment(tui::layout::Alignment::Center);
}

#[memoize::memoize]
fn make_problem_border(_unit: ()) -> Block<'static> {
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
            Span::from("[D]etails"),
            Span::from("  |  "),
            Span::from("[P]erformance "),
        ]),
        1 => Spans::from(vec![
            Span::from(" [T]est cases"),
            Span::from("  |  "),
            Span::styled("[S]etup:", selected_style),
            Span::from("  |  "),
            Span::from("[D]etails"),
            Span::from("  |  "),
            Span::from("[P]erformance "),
        ]),
        2 => Spans::from(vec![
            Span::from(" [T]est cases"),
            Span::from("  |  "),
            Span::from("[S]etup:"),
            Span::from("  |  "),
            Span::styled("[D]etails", selected_style),
            Span::from("  |  "),
            Span::from("[P]erformance "),
        ]),
        3 => Spans::from(vec![
            Span::from(" [T]est cases"),
            Span::from("  |  "),
            Span::from("[S]etup:"),
            Span::from("  |  "),
            Span::from("[D]etails"),
            Span::from("  |  "),
            Span::styled("[P]erformance ", selected_style),
        ]),
        _ => unreachable!(),
    };
    Block::default().borders(Borders::ALL).title(title)
}