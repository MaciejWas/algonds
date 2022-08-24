use crate::structure::common::ProblemDataKind;
use crate::structure::ui::CommandsView;
use crate::structure::ui::LastTestCaseView;
use crate::structure::ui::ProblemScreenLayout;
use crate::structure::ui::ProblemView;
use crate::structure::ui::TestCaseTable;
use crate::structure::View;
use crate::UIElement;
use tui::backend::Backend;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::Frame;

enum ProblemData {
    TestCases(TestCaseTable),
    LastFailedExample(LastTestCaseView),
    Commands(CommandsView),
}

impl UIElement for ProblemData {
    type ExpectedLayout = ProblemScreenLayout;

    fn setup(view: &View) -> Self {
        let to_show = view.curr_data();
        match to_show {
            ProblemDataKind::TestCases => Self::TestCases(TestCaseTable::setup(&view)),
            ProblemDataKind::Commands => Self::Commands(CommandsView::setup(&view)),
            ProblemDataKind::LastFailedExample => Self::LastFailedExample(LastTestCaseView::setup(&view))
        }
    }

    fn render<B: tui::backend::Backend>(self, frame: &mut Frame<B>, layout: &ProblemScreenLayout) {
        match self {
            Self::TestCases(widget) => widget.render(frame, layout),
            Self::Commands(widget) => widget.render(frame, layout),
            Self::LastFailedExample(widget) => widget.render(frame, layout),
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
        let problem_view_border = Block::default().borders(Borders::ALL).title("Solving");

        let selected_style = Style::default()
            .add_modifier(tui::style::Modifier::BOLD)
            .fg(tui::style::Color::Green);

        let title = match &self.run_data {
            ProblemData::Commands(_) => Spans::from(vec![
                Span::from(" [T]est cases"),
                Span::from("  |  "),
                Span::styled("[S]etup:", selected_style),
                Span::from("  |  "),
                Span::from("[F]ailed tests "),
            ]),
            ProblemData::TestCases(_) => Spans::from(vec![
                Span::styled(" [T]est cases", selected_style),
                Span::from("  |  "),
                Span::from("[S]etup:"),
                Span::from("  |  "),
                Span::from("[F]ailed tests "),
            ]),
            ProblemData::LastFailedExample(_) => Spans::from(vec![
                Span::from(" [T]est cases"),
                Span::from("  |  "),
                Span::from("[S]etup:"),
                Span::from("  |  "),
                Span::styled("[F]ailed tests ", selected_style),
            ]),
        };

        let problem_data_border = Block::default().borders(Borders::ALL).title(title);

        frame.render_widget(problem_view_border, layout.problem_window);
        frame.render_widget(problem_data_border, layout.data_window);

        self.problem_data.render(frame, &layout.problem);
        self.run_data.render(frame, &layout)
    }
}
