use crate::structure::ui::layouts::split_problem_preview;
use tui::layout::Rect;

#[derive(Clone, Copy)]
pub struct ProblemLayout {
    pub title: Rect,
    pub statement: Rect,
    pub example: Rect,
}

impl From<Rect> for ProblemLayout {
    fn from(term_size: Rect) -> Self {
        let (title, statement, example) = split_problem_preview(term_size);
        Self {
            title,
            statement,
            example,
        }
    }
}
