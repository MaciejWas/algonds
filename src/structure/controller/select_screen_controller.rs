use crate::structure::Controller;
use crate::structure::{common::Menu, controller::AfterEvent::*, ModelRef};
use crate::AfterEvent;
use crossterm::event::{Event, KeyCode};

pub struct SelectScreenController {
    model: ModelRef,
}

impl Controller for SelectScreenController {
    fn setup(model_ref: &ModelRef) -> Self {
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
        self.model.select_next(true);
        return DoRefresh;
    }

    fn prev_problem(&self) -> AfterEvent {
        self.model.select_next(false);
        return DoRefresh;
    }
}
