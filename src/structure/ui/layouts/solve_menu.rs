use tui::layout::Constraint;
use crate::structure::ui::ProblemLayout;
use crate::structure::ui::layouts::get_footnote;
use crate::structure::ui::layouts::PROBLEM_SCREEN_SPLIT;
use tui::layout::Layout;
use tui::layout::Rect;
use tui::layout::Direction;

const SPLIT_70_30: [Constraint; 2] = [
    Constraint::Percentage(70),
    Constraint::Percentage(30)
];

#[derive(Clone, Copy)]
pub struct ProblemScreenLayout {
    pub problem: ProblemLayout,
    pub data: Rect,
    pub footnote: Rect
}

impl From<Rect> for ProblemScreenLayout {
    fn from(term_size: Rect) -> ProblemScreenLayout {
        let problem_and_data = Layout::default()
            .constraints(SPLIT_70_30)
            .direction(Direction::Vertical)
            .margin(4)
            .split(term_size);

        let problem = problem_and_data[0];
        let data = problem_and_data[1];

        ProblemScreenLayout {
            problem: ProblemLayout::from(problem),
            data,
            footnote: get_footnote(term_size)
        }
    }
}