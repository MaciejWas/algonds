use crate::application::test_suite::TestSuite;
use crate::data::load;
use crate::application::common::*;
use crate::application::input_handler::InputHandler;
use crate::application::Settings;

use std::cell::Cell;
use std::cell::RefCell;
use std::iter::Iterator;
use std::rc::Rc;

use tui::widgets::ListState;

pub type Db = Vec<Rc<Problem>>;

pub struct Model {
    pub problem_data_tab: Cell<ProblemDataTab>,
    pub input_handler: InputHandler,
    pub settings: RefCell<Settings>,
    pub current_menu: Cell<Menu>,
    pub selected_test_case: Cell<usize>,

    db: Db,
    test_suite: TestSuite,
    list_state: RefCell<ListState>,
}

impl Model {
    pub fn new_ref(settings: Settings) -> Rc<Self> {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Rc::new(Model {
            problem_data_tab: Cell::default(),
            db: load(&settings.db_path),
            input_handler: InputHandler::default(),
            settings: RefCell::new(settings.clone()),
            current_menu: Cell::default(),
            test_suite: TestSuite::new(),
            list_state: RefCell::new(list_state),
            selected_test_case: Cell::default(),
        })
    }
    
    /// State of the scrollable list of problems
    pub fn get_list_state(&self) -> RefCell<ListState> {
        self.list_state.clone()
    }
    
    pub fn number_of_tests(&self) -> usize {
        self.test_suite.number_of_tests()
    }

    pub fn select_test_case(&self, dir: Direction) {
        let id = self.selected_test_case.get();
        let n_test_cases = self.number_of_tests();

        let next_id = if dir == Direction::Next {
            std::cmp::min(id + 1, n_test_cases - 1)
        } else {
            std::cmp::max(id as i32 - 1, 0) as usize
        };

        self.selected_test_case.set(next_id);
    }

    pub fn select_problem(&self, dir: Direction) {
        let id = self.list_state.borrow().selected().unwrap_or(0);
        let n_problems = self.total_problems();

        let next_id = if dir == Direction::Next {
            std::cmp::min(id + 1, n_problems - 1)
        } else if id > 0 {
            id - 1
        } else {
            id
        };

        self.list_state.borrow_mut().select(Some(next_id));
    }

    pub fn start_editing_field(&self, field: InputField) {
        let current_value = self.get_field(field);
        self.input_handler.edit_field(current_value, field)
    }

    pub fn go_to(&self, menu: Menu) {
        self.current_menu.set(menu);
        if menu == Menu::Solve {
            self.test_suite.set_test_cases_from(self.current_problem())
        }
    }

    pub fn cancel_editing_field(&self) {
        self.input_handler.finish();
    }

    pub fn cancel_run(&self) {
        self.test_suite.stop();
    }

    pub fn run_all_test_cases(&self) {
        let compile_script = self.settings.borrow().compilation_step.clone();
        let run_script = self.settings.borrow().run_step.clone();
        self.test_suite.run(compile_script, run_script);
    }

    pub fn current_problem(&self) -> Rc<Problem> {
        let id: usize = self.list_state.borrow().selected().unwrap_or(0);
        self.db.get(id).unwrap().clone()
    }

    pub fn get_problems(&self) -> Vec<Rc<Problem>> {
        self.db.iter().map(Rc::clone).collect()
    }

    pub fn get_field(&self, field: InputField) -> String {
        let settings = self.settings.borrow();
        match field {
            InputField::CompileCommand => settings.compilation_step.clone(),
            InputField::RunCommand => settings.run_step.clone(),
        }
    }

    pub fn total_problems(&self) -> usize {
        self.db.len()
    }

    pub fn finish_edit(&self) {
        let mut settings = self.settings.borrow_mut();
        let field = self.input_handler.current_field();
        let finished_input = self.input_handler.finish();
        if let Some(field) = field {
            match field {
                InputField::CompileCommand => settings.compilation_step = finished_input,
                InputField::RunCommand => settings.run_step = finished_input,
            }
        }
    }

    pub fn get_test_cases(&self) -> Vec<TestCaseStatus> {
        self.test_suite.get_test_cases()
    }

    pub fn details_for_selected_test_case(&self) -> (usize, TestCaseStatus) {
        let selected_id = self.selected_test_case.get();
        (
            selected_id,
            self.test_suite.get_nth_test_case(selected_id)
        )
    }

    pub fn check_for_changes(&self) -> bool {
        self.test_suite.check_for_changes()
    }

    pub fn kill_all_processes(&self) {
        self.test_suite.stop()
    }

    pub fn health_check(&self) {
    }
}
