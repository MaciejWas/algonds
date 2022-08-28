use crate::application::{controller::AfterEvent, controller::AfterEvent::*, Controller, Model};
use crossterm::event::{Event, KeyCode};
use std::rc::Rc;

pub struct InputController {
    model: Rc<Model>,
}

impl Controller for InputController {
    fn setup(model_ref: &Rc<Model>) -> Self {
        Self {
            model: model_ref.clone(),
        }
    }
    fn react_to_event(&self, event: Event) -> AfterEvent {
        if let Event::Key(key) = event {
            let _input_handler = &self.model.input_handler;
            return match key.code {
                KeyCode::Char(c) => self.add_to_input(c),
                KeyCode::Esc => self.cancel_input(),
                KeyCode::Backspace => self.pop_last_char(),
                KeyCode::Enter => self.finish_edit(),
                _ => NoRefresh,
            };
        }

        DoRefresh
    }
}

impl InputController {
    fn cancel_input(&self) -> AfterEvent {
        self.model.cancel_edit();
        DoRefresh
    }

    fn add_to_input(&self, c: char) -> AfterEvent {
        self.model.input_handler.add(c);
        DoRefresh
    }

    fn finish_edit(&self) -> AfterEvent {
        self.model.finish_edit();
        DoRefresh
    }

    fn pop_last_char(&self) -> AfterEvent {
        self.model.input_handler.pop();
        DoRefresh
    }
}
