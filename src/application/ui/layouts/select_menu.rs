use tui::layout::Layout;
use crate::application::ui::ProblemLayout;
use tui::{layout::{Rect, Constraint, Direction}};

use super::{add_margin, get_footnote, split_left_right};

const SPLIT_70_30: [Constraint; 2] = [Constraint::Percentage(70), Constraint::Percentage(30)];

#[derive(Clone, Copy)]
pub struct SelectScreenLayout {
    pub problem_list_outline: Rect,
    pub problem_list: Rect,

    pub problem_preview_outline: Rect,
    pub problem_preview: ProblemLayout,
    pub footnote: Rect,
}

impl From<Rect> for SelectScreenLayout {
    fn from(term_size: Rect) -> SelectScreenLayout {
        let list_and_preview = Layout::default()
            .constraints(SPLIT_70_30)
            .direction(Direction::Vertical)
            .margin(4)
            .split(term_size);

        let problem_list_outline = list_and_preview[0];
        let problem_list = add_margin(problem_list_outline);

        let problem_preview_outline = list_and_preview[1];
        let problem_preview = ProblemLayout::from(problem_preview_outline);
        let footnote = get_footnote(term_size);

        SelectScreenLayout {
            problem_list_outline,
            problem_list,
            problem_preview_outline,
            problem_preview,
            footnote,
        }
    }
}
