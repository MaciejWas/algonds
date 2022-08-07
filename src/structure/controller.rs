use crate::structure::common::Problem;
use crate::structure::controller::EventResult::*;
use crate::structure::view::{Menu, View};
use crate::structure::Difficulty;
use crate::structure::Model;
use crate::structure::ModelRef;
use crate::structure::Run;
use crate::structure::Settings;
use crate::AppState;
use crossterm::event::{Event, KeyCode};
use std::cell::RefCell;
use std::rc::Rc;

pub enum EventResult {
    Quit,
    DoRefresh,
    NoRefresh,
}
impl EventResult {
    pub fn is_quit(&self) -> bool {
        if let Quit = self {
            true
        } else {
            false
        }
    }
}

impl Default for EventResult {
    fn default() -> Self {
        DoRefresh
    }
}

pub struct Controller {
    model: ModelRef,
}

impl From<&ModelRef> for Controller {
    fn from(model: &ModelRef) -> Self {
        Self {
            model: Rc::clone(model),
        }
    }
}

impl Controller {
    pub fn react_to_event(&self, event: Event) -> EventResult {
        if let Event::Key(key) = event {
            let menu = self.model.menu.borrow_mut();
            match *menu {
                Menu::Select => match key.code {
                    KeyCode::Char('j') => {
                        let curr_id = self.model.curr_prob_id.get();
                        if curr_id < self.model.total_problems() - 1
                        {
                            self.model.curr_prob_id.set(curr_id + 1);
                            return DoRefresh;
                        }
                    }
                    KeyCode::Char('k') => {
                        let curr_id = self.model.curr_prob_id.get();
                        if curr_id > 0 {
                            self.model.curr_prob_id.set(curr_id - 1);
                            return DoRefresh;
                        }
                    }
                    KeyCode::Char('q') => return Quit,
                    _ => {}
                },
                _ => {}
            }
            return NoRefresh;
        }
        return NoRefresh;
    }
}
