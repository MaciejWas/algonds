use crate::application::Problem;
use crate::application::{common::*, Model};
use std::cell::RefCell;
use std::rc::Rc;
use tui::widgets::ListState;

pub struct View {
    model: Rc<Model>,
}

impl From<&Rc<Model>> for View {
    fn from(model: &Rc<Model>) -> Self {
        Self {
            model: Rc::clone(model),
        }
    }
}

impl View {
    pub fn get_list_state(&self) -> RefCell<ListState> {
        self.model.get_list_state()
    }

    pub fn get_problems_to_select(&self) -> Vec<Rc<Problem>> {
        self.model.get_problems()
    }

    pub fn get_cursor(&self) -> String {
        let sign = if self.model.settings.borrow().pretty {
            " 🡆  "
        } else {
            " -> "
        };
        sign.to_string()
    }

    pub fn curr_data(&self) -> ProblemDataKind {
        self.model.problem_data_kind.get()
    }

    pub fn curr_menu(&self) -> Menu {
        self.model.menu.get()
    }

    pub fn current_problem<'a>(&self) -> Rc<Problem> {
        self.model.current_problem()
    }

    pub fn curr_field(&self) -> Option<InputField> {
        self.model.input_handler.current_field()
    }

    pub fn compile_command_view(&self) -> String {
        if self.model.input_handler.is_in_input_mode() {
            if self.model.input_handler.current_field() == Some(InputField::CompileCommand) {
                return self.model.input_handler.curr_input() + &"|";
            }
        }

        return self.model.get_field(InputField::CompileCommand);
    }

    pub fn run_command_view(&self) -> String {
        if self.model.input_handler.is_in_input_mode() {
            if self.model.input_handler.current_field() == Some(InputField::RunCommand) {
                return self.model.input_handler.curr_input() + &"|";
            }
        }

        return self.model.get_field(InputField::RunCommand);
    }

    pub fn get_test_cases(&self) -> Vec<TestCaseStatus> {
        self.model.get_test_cases()
    }

    pub fn details_for_selected_test_case(&self) -> (usize, TestCaseStatus) {
        self.model.details_for_selected_test_case()
    }

    pub fn check_for_changes(&self) -> bool {
        self.model.check_for_changes()
    }

    pub fn get_n_problems(&self) -> usize {
        self.model.number_of_problems()
    }
}