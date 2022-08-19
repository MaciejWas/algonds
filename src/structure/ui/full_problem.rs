
use tui::backend::Backend;
use tui::Frame;
use crate::structure::View;
use crate::structure::ui::ProblemScreenLayout;
use crate::UIElement;
use crate::structure::ui::TestCaseTable;
use crate::structure::ui::ProblemView;

type ProblemData<'a> = TestCaseTable<'a>;

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
        
        self.problem_data.render(frame, &layout.problem);
        self.run_data.render(frame, &layout)
    }
}