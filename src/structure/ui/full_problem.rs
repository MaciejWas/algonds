use crate::structure::ui::LastTestCaseView;
use crate::structure::ui::CommandsView;
use tui::widgets::Paragraph;
use tui::widgets::Borders;
use tui::widgets::Block;
use crate::structure::common::ProblemDataKind;
use tui::backend::Backend;
use tui::Frame;
use crate::structure::View;
use crate::structure::ui::ProblemScreenLayout;
use crate::UIElement;
use crate::structure::ui::TestCaseTable;
use crate::structure::ui::ProblemView;

enum ProblemData<'a> {
    TestCases(TestCaseTable<'a>),
    LastFailedExample(LastTestCaseView),
    Commands(CommandsView)
}

impl<'a> UIElement for ProblemData<'a> {
    type ExpectedLayout = ProblemScreenLayout;
    
    fn setup(view: &View) -> Self {
        let to_show = view.curr_data();
        match to_show {
            ProblemDataKind::TestCases => Self::TestCases(TestCaseTable::setup(&view)),
            ProblemDataKind::Commands => Self::Commands(CommandsView::setup(&view)),
            ProblemDataKind::LastFailedExample => Self::LastFailedExample(LastTestCaseView::setup(&view)),
        }
    }

    
    fn render<B: tui::backend::Backend>(self, frame: &mut Frame<B>, layout: &ProblemScreenLayout)  { 
        match self {
            Self::TestCases(widget) => widget.render(frame, layout),
            Self::Commands(widget) => widget.render(frame, layout),
            Self::LastFailedExample(widget) => widget.render(frame, layout),
        }
    }
}



pub struct FullProblem<'a> {
    problem_data: ProblemView<'a>,
    run_data: ProblemData<'a>,
}


impl<'a> UIElement for FullProblem<'a> {
    type ExpectedLayout = ProblemScreenLayout;
    fn setup(view: &View) -> Self { 
        let problem_data = ProblemView::setup(view);
        let run_data = ProblemData::setup(view);
        Self { problem_data, run_data }
    }
    fn render<B>(self, frame: &mut Frame<B>, layout: &ProblemScreenLayout) where B: Backend { 
        let problem_view_border = Block::default()
            .borders(Borders::ALL)
            .title("Solving");

        let problem_data_border = Block::default()
            .borders(Borders::ALL)
            .title("[T]est cases / [S]etup / [L]ast failed test");

        frame.render_widget(problem_view_border, layout.problem_window);
        frame.render_widget(problem_data_border, layout.data_window);
        
        self.problem_data.render(frame, &layout.problem);
        self.run_data.render(frame, &layout)
    }
}