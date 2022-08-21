use tui::layout::Layout;
use tui::layout::Constraint;
use tui::layout::Rect;
use tui::layout::Direction;

mod help_menu;
mod select_menu;
mod solve_menu;
mod problem_layout;

pub use help_menu::HelpScreenLayout;
pub use select_menu::SelectScreenLayout;
pub use solve_menu::ProblemScreenLayout;
pub use problem_layout::ProblemLayout;

const SPLIT_HELP: [Constraint; 3] = [
    Constraint::Percentage(33),
    Constraint::Percentage(33),
    Constraint::Percentage(33),
];

const SELECT_PROBLEM_WINDOWS_CONSTRAINTS: [Constraint; 2] = [
    Constraint::Percentage(40),
    Constraint::Percentage(50)
];

const FOOTNOTE_CONSTRAINTS: [Constraint; 2] = [
    Constraint::Percentage(100),
    Constraint::Min(1),
];

const PROBLEM_PREVIEW_CONSTRAINTS: [Constraint; 3] = [
    Constraint::Min(1),
    Constraint::Percentage(40),
    Constraint::Percentage(40)
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
    Layout::default().constraints([Constraint::Percentage(100)]).margin(2).split(area)[0]
}

