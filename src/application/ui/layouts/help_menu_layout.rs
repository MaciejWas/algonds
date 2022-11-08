use tui::layout::Constraint;
use tui::layout::Layout;
use tui::layout::Rect;

const SPLIT_HELP: [Constraint; 3] = [
    Constraint::Percentage(33),
    Constraint::Percentage(33),
    Constraint::Percentage(34),
];

#[derive(Clone, Copy)]
pub struct HelpScreenLayout {
    pub outer_window: Rect,
    pub general_help: Rect,
    pub select_help: Rect,
    pub solve_help: Rect,
}

impl From<Rect> for HelpScreenLayout {
    fn from(term_size: Rect) -> Self {
        let outer_window = Layout::default()
            .margin(3)
            .constraints(vec![Constraint::Percentage(100)])
            .split(term_size)[0];
        let fields = Layout::default()
            .constraints(SPLIT_HELP)
            .split(outer_window);
        HelpScreenLayout {
            outer_window,
            general_help: fields[0],
            select_help: fields[1],
            solve_help: fields[2],
        }
    }
}
