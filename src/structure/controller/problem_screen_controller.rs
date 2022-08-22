use crate::structure::{
    common::Menu, controller::input_controller::InputController, controller::AfterEvent,
    controller::AfterEvent::*, Controller, InputField, ModelRef, ProblemDataKind,
};

use crossterm::event::{Event, KeyCode};

pub struct ProblemScreenController {
    model: ModelRef,
    input_controller: InputController,
}

impl Controller for ProblemScreenController {
    fn setup(model_ref: &ModelRef) -> Self {
        Self {
            model: model_ref.clone(),
            input_controller: InputController::setup(&model_ref),
        }
    }
    fn react_to_event(&self, event: Event) -> AfterEvent {
        if self.model.input_handler.is_in_input_mode() {
            return self.input_controller.react_to_event(event);
        }

        if let Event::Key(key) = event {
            return match key.code {
                KeyCode::Char('c') => self.edit(InputField::CompileCommand),
                KeyCode::Char('r') => self.edit(InputField::RunCommand),
                KeyCode::Char('s') => self.display_under_problem(ProblemDataKind::Commands),
                KeyCode::Char('t') => self.display_under_problem(ProblemDataKind::TestCases),
                KeyCode::Char('f') => {
                    self.display_under_problem(ProblemDataKind::LastFailedExample)
                }
                KeyCode::Char('q') => self.change_menu(Menu::Select),
                KeyCode::Enter => self.run_test_cases(),
                _ => NoRefresh,
            };
        }

        NoRefresh
    }
}

impl ProblemScreenController {
    fn display_under_problem(&self, problem_data_kind: ProblemDataKind) -> AfterEvent {
        self.model.problem_data_kind.set(problem_data_kind);
        DoRefresh
    }

    fn edit(&self, field: InputField) -> AfterEvent {
        self.display_under_problem(ProblemDataKind::Commands);
        self.model.edit_field(field);
        DoRefresh
    }

    fn change_menu(&self, new_menu: Menu) -> AfterEvent {
        self.model.go_to(new_menu);
        DoRefresh
    }

    fn run_test_cases(&self) -> AfterEvent {
        self.model.reset_test_cases();
        self.model.run_test_cases();
        DoRefresh
    }
}
