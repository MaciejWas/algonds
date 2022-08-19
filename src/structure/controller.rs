use crate::event::KeyEvent;
use crate::structure::{
    controller::AfterEvent::*, common::Menu, InputField, InputField::*, ModelRef,
};
use crossterm::event::{Event, KeyCode};
use std::rc::Rc;

#[derive(Eq, PartialEq, Debug)]
pub enum AfterEvent {
    Quit,
    DoRefresh,
    NoRefresh,
}

impl AfterEvent {
    pub fn and(self, other: AfterEvent) -> Self {
        match (self, other) {
            (Quit, _) => Quit,
            (DoRefresh, Quit) => Quit,
            (DoRefresh, _) => DoRefresh,
            (NoRefresh, other) => other,
        }
    }

    pub fn is_quit(&self) -> bool {
        self.eq(&Self::Quit)
    }
}

impl Default for AfterEvent {
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
    fn next_problem(&self) -> AfterEvent {
        self.model.select_next(true);
        return DoRefresh;
    }

    fn prev_problem(&self) -> AfterEvent {
        self.model.select_next(false);
        return DoRefresh;
    }

    fn universal_actions(&self, key: KeyEvent) -> AfterEvent {
        if let KeyCode::Char('q') = key.code {
            return Quit;
        }

        if let KeyCode::Char('c') = key.code && key.modifiers == crossterm::event::KeyModifiers::CONTROL{
            return Quit
        }

        if let KeyCode::Char('h') = key.code {
            self.change_menu(Menu::Help);
            return DoRefresh;
        }

        NoRefresh
    }

    fn handle_select_menu(&self, key: KeyEvent) -> AfterEvent {
        match key.code {
            KeyCode::Char('j') => self.next_problem(),
            KeyCode::Up => self.prev_problem(),
            KeyCode::Down => self.next_problem(),
            KeyCode::Char('k') => self.prev_problem(),
            KeyCode::Enter => self.change_menu(Menu::Solve),
            _ => self.universal_actions(key),
        }
    }

    fn handle_help_menu(&self) -> AfterEvent {
        self.change_menu(Menu::Select)
    }

    fn handle_input(&self, key: KeyEvent) -> AfterEvent {
        let input_handler = &self.model.input_handler;
        match key.code {
            KeyCode::Char(c) => input_handler.add(c),
            KeyCode::Backspace => input_handler.pop(),
            KeyCode::Enter => self.model.finish_edit(),
            _ => return NoRefresh
        };

        DoRefresh
    }

    fn edit(&self, field: InputField) -> AfterEvent {
        self.model.input_handler.edit_field(field);
        DoRefresh
    }

    fn handle_solve_menu(&self, key: KeyEvent) -> AfterEvent {
        if self.model.input_handler.is_in_input_mode() {
            return self.handle_input(key);
        }

        match key.code {
            KeyCode::Char('c') => self.edit(CompileCommand),
            KeyCode::Char('r') => self.edit(RunCommand),
            KeyCode::Char('q') => self.change_menu(Menu::Select),
            _ => self.universal_actions(key),
        }
    }

    pub fn react_to_event(&self, event: Event) -> AfterEvent {
        if let Event::Resize(_, _) = event {
            return DoRefresh;
        }

        if let Event::Key(key) = event {
            return match self.model.menu.get() {
                Menu::Help => self.handle_help_menu(),
                Menu::Select => self.handle_select_menu(key),
                Menu::Solve => self.handle_solve_menu(key),
                _ => self.universal_actions(key),
            };
        }
        return NoRefresh;
    }

    fn change_menu(&self, new_menu: Menu) -> AfterEvent {
        self.model.menu.set(new_menu);
        DoRefresh
    }
}
