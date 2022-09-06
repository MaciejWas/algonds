use crate::application::{
    common::{Direction, InputField, Menu, ProblemDataTab},
    controller::input_controller::InputController,
    controller::AfterEvent,
    controller::AfterEvent::*,
    Controller, Model,
};
use crossterm::event::{Event, KeyCode};
use std::rc::Rc;

pub struct ProblemScreenController {
    model: Rc<Model>,
    input_controller: InputController,
}

impl Controller for ProblemScreenController {
    fn setup(model_ref: &Rc<Model>) -> Self {
        Self {
            model: model_ref.clone(),
            input_controller: InputController::setup(model_ref),
        }
    }
    fn react_to_event(&self, event: Event) -> AfterEvent {
        if self.model.input_handler.is_in_input_mode() {
            return self.input_controller.react_to_event(event);
        }

        if let Event::Key(key) = event {
            if self.model.problem_data_tab.get() == ProblemDataTab::Details {
                match key.code {
                    KeyCode::Left => return self.select_prev_test_case(),
                    KeyCode::Right => return self.select_next_test_case(),
                    _ => {}
                }
            }

            return match key.code {
                KeyCode::Char('c') => self.edit(InputField::CompileCommand),
                KeyCode::Char('r') => self.edit(InputField::RunCommand),
                KeyCode::Char('s') => self.display_under_problem(ProblemDataTab::Commands),
                KeyCode::Char('t') => self.display_under_problem(ProblemDataTab::TestCases),
                KeyCode::Char('d') => self.display_under_problem(ProblemDataTab::Details),
                KeyCode::Char('p') => self.display_under_problem(ProblemDataTab::Performance),
                KeyCode::Char('q') => self.change_menu(Menu::Select),
                KeyCode::Enter => self.run_test_cases(),
                KeyCode::Backspace => self.cancel_test_cases(),
                _ => NoRefresh,
            };
        }

        NoRefresh
    }
}

impl ProblemScreenController {
    fn cancel_test_cases(&self) -> AfterEvent {
        self.model.cancel_run();
        DoRefresh
    }

    fn display_under_problem(&self, problem_data_kind: ProblemDataTab) -> AfterEvent {
        self.model.problem_data_tab.set(problem_data_kind);
        DoRefresh
    }

    fn edit(&self, field: InputField) -> AfterEvent {
        self.display_under_problem(ProblemDataTab::Commands);
        self.model.start_editing_field(field);
        DoRefresh
    }

    fn change_menu(&self, new_menu: Menu) -> AfterEvent {
        self.model.go_to(new_menu);
        DoRefresh
    }

    fn run_test_cases(&self) -> AfterEvent {
        self.model.run_all_test_cases();
        DoRefresh
    }

    fn select_next_test_case(&self) -> AfterEvent {
        self.model.select_test_case(Direction::Next);
        DoRefresh
    }

    fn select_prev_test_case(&self) -> AfterEvent {
        self.model.select_test_case(Direction::Previous);
        DoRefresh
    }
}
