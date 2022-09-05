use crate::application::{
    common::{Direction, Menu},
    controller::AfterEvent,
    controller::AfterEvent::*,
    Controller, Model,
};
use crossterm::event::{Event, KeyCode};
use std::rc::Rc;

pub struct SelectScreenController {
    model: Rc<Model>,
}

impl Controller for SelectScreenController {
    fn setup(model_ref: &Rc<Model>) -> Self {
        Self {
            model: model_ref.clone(),
        }
    }
    fn react_to_event(&self, event: Event) -> AfterEvent {
        if let Event::Key(key) = event {
            return match key.code {
                KeyCode::Char('j') => self.next_problem(),
                KeyCode::Up => self.prev_problem(),
                KeyCode::Down => self.next_problem(),
                KeyCode::Char('k') => self.prev_problem(),
                KeyCode::Enter => self.change_menu(Menu::Solve),
                _ => NoRefresh,
            };
        };
        NoRefresh
    }
}

impl SelectScreenController {
    fn change_menu(&self, new_menu: Menu) -> AfterEvent {
        self.model.go_to(new_menu);
        DoRefresh
    }

    fn next_problem(&self) -> AfterEvent {
        self.model.select_problem(Direction::Next);
        return DoRefresh;
    }

    fn prev_problem(&self) -> AfterEvent {
        self.model.select_problem(Direction::Previous);
        return DoRefresh;
    }
}
