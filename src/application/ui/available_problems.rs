use crate::application::common::*;
use crate::application::ui::SelectScreenLayout;
use crate::application::ui::UIElement;
use crate::application::View;
use std::cell::RefCell;
use std::rc::Rc;
use tui::widgets::Paragraph;
use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

pub struct AvailableProblems<'a> {
    items: Vec<ListItem<'a>>,
    cursor: String,
    list_state: RefCell<ListState>,
}

impl<'a> UIElement for AvailableProblems<'a> {
    type ExpectedLayout = SelectScreenLayout;

    fn setup(view: &View) -> Self {
        let list_state = view.get_list_state();
        let cursor = view.get_cursor();
        let problems: Vec<Rc<Problem>> = view.get_problems_to_select();
        let items = problems
            .into_iter()
            .map(|prob| ListItem::new(Spans::from(Span::from(prob.name.clone()))))
            .collect();
        Self {
            items,
            cursor,
            list_state,
        }
    }

    fn render<B: Backend>(self, frame: &mut Frame<B>, layout: &SelectScreenLayout) {
        let left_border = Block::default()
            .borders(Borders::ALL)
            .title("Available Problems");
        let right_border = Block::default().borders(Borders::ALL).title("Selected");
        let list = List::new(self.items)
            .highlight_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(&self.cursor);

        frame.render_stateful_widget(list, layout.problem_list, &mut self.list_state.into_inner());
        frame.render_widget(left_border, layout.problem_list_outline);
        frame.render_widget(right_border, layout.problem_preview_outline);
        frame.render_widget(
            Paragraph::new("q - quit,   h - help,   enter - select problem,   use arrows to navigate").alignment(tui::layout::Alignment::Center), 
            layout.footnote)
    }
}
