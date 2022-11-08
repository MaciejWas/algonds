use crate::application::{common::Menu, controller::AfterEvent::*, Model};
use crate::event::KeyEvent;
use crossterm::event::{Event, KeyCode};
use std::rc::Rc;

mod input_controller;
mod problem_screen_controller;
mod select_screen_controller;

use problem_screen_controller::ProblemScreenController;
use select_screen_controller::SelectScreenController;

#[derive(Eq, PartialEq, Debug)]
pub enum AfterEvent {
    Quit,
    DoRefresh,
    NoRefresh,
}

impl Default for AfterEvent {
    fn default() -> Self {
        DoRefresh
    }
}

pub trait Controller {
    fn setup(model_ref: &Rc<Model>) -> Self;
    fn react_to_event(&self, event: Event) -> AfterEvent;
}

pub struct MainController {
    model: Rc<Model>,
    select_screen_controller: SelectScreenController,
    problem_screen_controller: ProblemScreenController,
}

impl Controller for MainController {
    fn setup(model: &Rc<Model>) -> Self {
        Self {
            model: Rc::clone(model),
            select_screen_controller: SelectScreenController::setup(model),
            problem_screen_controller: ProblemScreenController::setup(model),
        }
    }

    fn react_to_event(&self, event: Event) -> AfterEvent {
        if let Event::Resize(_, _) = event {
            return DoRefresh;
        }

        if let Event::Key(key) = event {
            if let KeyCode::PageUp = key.code {
                self.model.health_check();
            }

            let result = match self.model.current_menu.get() {
                Menu::Help => self.handle_help_menu(),
                Menu::Select => self.select_screen_controller.react_to_event(event),
                Menu::Solve => self.problem_screen_controller.react_to_event(event),
            };

            if result == AfterEvent::NoRefresh {
                return self.universal_actions(key);
            }

            return result;
        }
        NoRefresh
    }
}

impl MainController {
    fn universal_actions(&self, key: KeyEvent) -> AfterEvent {
        if let KeyCode::Char('q') = key.code {
            return Quit;
        }

        if let KeyCode::Char('c') = key.code {
            if key.modifiers == crossterm::event::KeyModifiers::CONTROL {
                return Quit
            }
        }

        if let KeyCode::Char('h') = key.code {
            self.change_menu(Menu::Help);
            return DoRefresh;
        }

        NoRefresh
    }

    fn handle_help_menu(&self) -> AfterEvent {
        self.change_menu(Menu::Select)
    }

    fn change_menu(&self, new_menu: Menu) -> AfterEvent {
        self.model.go_to(new_menu);
        DoRefresh
    }

    pub fn kill_all_processes(&self) {
        self.model.kill_all_processes()
    }
}
