use crate::event::KeyEvent;
use crate::structure::controller::EventResult::*;
use crate::structure::view::{Menu};
use crate::structure::ModelRef;
use crossterm::event::{Event, KeyCode};
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
    fn next_problem(&self) -> EventResult {
        let curr_id = self.model.curr_prob_id.get();
            if curr_id < self.model.total_problems() - 1
            {
                self.model.curr_prob_id.set(curr_id + 1);
                return DoRefresh;
            }
            return NoRefresh
    }

    fn prev_problem(&self) -> EventResult {
        let curr_id = self.model.curr_prob_id.get();
        if curr_id > 0 {
            self.model.curr_prob_id.set(curr_id - 1);
            return DoRefresh;
        }
        return NoRefresh
    }

    fn universal_actions(&self, key: KeyEvent) -> EventResult {
        if let KeyCode::Char('q') = key.code {
            return Quit
        }

        if let KeyCode::Char('c') = key.code && key.modifiers == crossterm::event::KeyModifiers::CONTROL{
            return Quit
        }

        if let KeyCode::Char('h') = key.code {
            self.change_menu(Menu::Help);
            return DoRefresh
        }

        NoRefresh
    }

    fn handle_select_menu(&self, key: KeyEvent) -> EventResult {
        match key.code {
            KeyCode::Char('j') => self.next_problem(),
            KeyCode::Char('k') => self.prev_problem(),
            KeyCode::Enter     => self.change_menu(Menu::Solve),
            _                  => self.universal_actions(key)
        }
    }

    fn handle_help_menu(&self) -> EventResult {
        self.change_menu(Menu::Select)
    }

    fn handle_solve_menu(&self, key: KeyEvent) -> EventResult {
        match key.code {
            // KeyCode::Char('j') => self.next_problem(),
            // KeyCode::Char('k') => self.prev_problem(),
            KeyCode::Char('q') => self.change_menu(Menu::Select),
            _                  => self.universal_actions(key)
        }
    }

    pub fn react_to_event(&self, event: Event) -> EventResult {
        if let Event::Key(key) = event {
            return match self.model.menu.get() {
                Menu::Help   => self.handle_help_menu(),
                Menu::Select => self.handle_select_menu(key),
                Menu::Solve  => self.handle_solve_menu(key),
                _            => self.universal_actions(key)
            }
        }
        return NoRefresh;
    }

    fn change_menu(&self, new_menu: Menu) -> EventResult {
        self.model.menu.set(new_menu);
        DoRefresh
    }
}
