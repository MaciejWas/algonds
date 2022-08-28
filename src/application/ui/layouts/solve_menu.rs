use crate::application::ui::layouts::add_margin;
use crate::application::ui::layouts::get_footnote;
use crate::application::ui::ProblemLayout;
use tui::layout::Constraint;
use tui::layout::Direction;
use tui::layout::Layout;
use tui::layout::Rect;

const SPLIT_70_30: [Constraint; 2] = [Constraint::Percentage(70), Constraint::Percentage(30)];

#[derive(Clone, Copy)]
pub struct ProblemScreenLayout {
    pub problem_window: Rect,
    pub problem: ProblemLayout,
    pub data_window: Rect,
    pub data: Rect,
    pub footnote: Rect,
}

impl From<Rect> for ProblemScreenLayout {
    fn from(term_size: Rect) -> ProblemScreenLayout {
        let problem_and_data = Layout::default()
            .constraints(SPLIT_70_30)
            .direction(Direction::Vertical)
            .margin(4)
            .split(term_size);

        let problem_window = problem_and_data[0];
        let problem_view = add_margin(problem_window);
        let data_window = problem_and_data[1];
        let data = add_margin(data_window);

        ProblemScreenLayout {
            problem_window,
            data_window,
            problem: ProblemLayout::from(problem_view),
            data,
            footnote: get_footnote(term_size),
        }
    }
}
