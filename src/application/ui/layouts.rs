use tui::layout::Constraint;
use tui::layout::Direction;
use tui::layout::Layout;
use tui::layout::Rect;

mod help_menu_layout;
mod select_menu_layout;
mod problem_menu_layout;

pub use help_menu_layout::HelpScreenLayout;
pub use select_menu_layout::SelectScreenLayout;
pub use problem_menu_layout::{ProblemMenuLayout, ProblemStatementLayout};

const FOOTNOTE_CONSTRAINTS: [Constraint; 2] = [Constraint::Percentage(97), Constraint::Min(1)];

const PROBLEM_PREVIEW_CONSTRAINTS: [Constraint; 3] = [
    Constraint::Min(1),
    Constraint::Percentage(40),
    Constraint::Percentage(40),
];

fn split_problem_preview(area: Rect) -> (Rect, Rect, Rect) {
    let fields = Layout::default()
        .constraints(PROBLEM_PREVIEW_CONSTRAINTS)
        .direction(Direction::Vertical)
        .margin(3)
        .split(area);
    (fields[0], fields[1], fields[2])
}

fn get_footnote(area: Rect) -> Rect {
    Layout::default()
        .constraints(FOOTNOTE_CONSTRAINTS)
        .direction(Direction::Vertical)
        .split(area)[1]
}

fn add_margin(area: Rect) -> Rect {
    Layout::default()
        .constraints([Constraint::Percentage(100)])
        .margin(2)
        .split(area)[0]
}
