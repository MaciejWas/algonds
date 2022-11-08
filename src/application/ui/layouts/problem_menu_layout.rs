use crate::application::ui::layouts::add_margin;
use crate::application::ui::layouts::get_footnote;
use crate::application::ui::layouts::split_problem_preview;
use tui::layout::Constraint;
use tui::layout::Direction;
use tui::layout::Layout;
use tui::layout::Rect;


const SPLIT_65_35: [Constraint; 2] = [Constraint::Percentage(65), Constraint::Percentage(35)];

#[derive(Clone, Copy)]
pub struct ProblemStatementLayout {
    pub title: Rect,
    pub statement: Rect,
    pub example: Rect,
}

impl From<Rect> for ProblemStatementLayout {
    fn from(term_size: Rect) -> Self {
        let (title, statement, example) = split_problem_preview(term_size);
        Self {
            title,
            statement,
            example,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ProblemMenuLayout {
    pub problem_window: Rect,
    pub problem: ProblemStatementLayout,

    pub problem_tabs_window: Rect,
    pub problem_tabs: Rect,

    pub footnote: Rect,
}

impl From<Rect> for ProblemMenuLayout {
    fn from(term_size: Rect) -> ProblemMenuLayout {
        let problem_and_data = Layout::default()
            .constraints(SPLIT_65_35)
            .direction(Direction::Vertical)
            .margin(4)
            .split(term_size);

        let problem_window = problem_and_data[0];
        let problem_tabs_window = problem_and_data[1];

        let problem_view = add_margin(problem_window);
        let problem_tabs = add_margin(problem_tabs_window);

        ProblemMenuLayout {
            problem_window,
            problem_tabs_window,
            problem: ProblemStatementLayout::from(problem_view),
            problem_tabs,
            footnote: get_footnote(term_size),
        }
    }
}
