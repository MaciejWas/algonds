use crate::structure::common::*;
use crate::structure::ui::SelectScreenLayout;
use crate::structure::ui::UIElement;
use crate::structure::View;
use std::cell::RefCell;
use std::rc::Rc;
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
            .map(|prob| ListItem::new(Spans::from(Span::from(prob.problem_name.clone()))))
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

        frame.render_stateful_widget(list, layout.rows, &mut self.list_state.into_inner());
        frame.render_widget(left_border, layout.left_window);
        frame.render_widget(right_border, layout.right_window);
    }
}
