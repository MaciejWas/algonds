use crate::structure::ui::ProblemLayout;
use tui::layout::Rect;

use super::{add_margin, get_footnote, split_left_right};

#[derive(Clone, Copy)]
pub struct SelectScreenLayout {
    pub left_window: Rect,
    pub right_window: Rect,
    pub rows: Rect,
    pub problem_preview: ProblemLayout,
    pub footnote: Rect,
}

impl From<Rect> for SelectScreenLayout {
    fn from(term_size: Rect) -> SelectScreenLayout {
        let (left_window, right_window) = split_left_right(term_size);
        let problem_preview = ProblemLayout::from(right_window);
        let footnote = get_footnote(term_size);
        let rows = add_margin(left_window);
        SelectScreenLayout {
            left_window,
            right_window,
            problem_preview,
            footnote,
            rows,
        }
    }
}
