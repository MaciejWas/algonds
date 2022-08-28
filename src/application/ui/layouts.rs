use tui::layout::Constraint;
use tui::layout::Direction;
use tui::layout::Layout;
use tui::layout::Rect;

mod help_menu;
mod problem_layout;
mod select_menu;
mod solve_menu;

pub use help_menu::HelpScreenLayout;
pub use problem_layout::ProblemLayout;
pub use select_menu::SelectScreenLayout;
pub use solve_menu::ProblemScreenLayout;

const SPLIT_HELP: [Constraint; 3] = [
    Constraint::Percentage(33),
    Constraint::Percentage(33),
    Constraint::Percentage(33),
];

const SELECT_PROBLEM_WINDOWS_CONSTRAINTS: [Constraint; 2] =
    [Constraint::Percentage(40), Constraint::Percentage(50)];

const FOOTNOTE_CONSTRAINTS: [Constraint; 2] = [Constraint::Percentage(100), Constraint::Min(1)];

const PROBLEM_PREVIEW_CONSTRAINTS: [Constraint; 3] = [
    Constraint::Min(1),
    Constraint::Percentage(40),
    Constraint::Percentage(40),
];

fn split_left_right(area: Rect) -> (Rect, Rect) {
    let windows = Layout::default()
        .constraints(SELECT_PROBLEM_WINDOWS_CONSTRAINTS)
        .direction(Direction::Horizontal)
        .margin(3)
        .split(area);
    (windows[0], windows[1])
}

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
