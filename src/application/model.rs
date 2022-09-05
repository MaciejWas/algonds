use crate::application::common::Menu;
use crate::application::common::*;
use crate::application::input_handler::InputHandler;
use crate::application::test_runner::CodeRunner;
use crate::application::Settings;
use std::cell::Cell;
use std::cell::RefCell;
use std::iter::Iterator;
use std::rc::Rc;
use std::sync::mpsc::SendError;
use tui::widgets::ListState;

pub type Db = Vec<Rc<Problem>>;

pub struct Model {
    pub problem_data_kind: Cell<ProblemDataKind>,
    pub input_handler: InputHandler,
    pub settings: RefCell<Settings>,
    pub menu: Cell<Menu>,
    pub selected_test_case: Cell<usize>,

    db: Db,
    code_runner: CodeRunner,
    new_test_cases_arrived: Cell<bool>,
    test_cases: RefCell<Vec<TestCaseStatus>>,
    list_state: RefCell<ListState>,
}

impl Model {
    pub fn new_ref(settings: Settings) -> Rc<Self> {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Rc::new(Model {
            problem_data_kind: Cell::default(),
            db: Model::load(&settings.db_path),
            code_runner: CodeRunner::default(),
            input_handler: InputHandler::default(),
            settings: RefCell::new(settings.clone()),
            menu: Cell::default(),
            test_cases: RefCell::default(),
            list_state: RefCell::new(list_state),
            new_test_cases_arrived: Cell::default(),
            selected_test_case: Cell::default(),
        })
    }

    pub fn get_list_state(&self) -> RefCell<ListState> {
        self.list_state.clone()
    }

    pub fn number_of_problems(&self) -> usize {
        self.test_cases.borrow().len()
    }

    pub fn select_test_case(&self, dir: Direction) {
        let id = self.selected_test_case.get();
        let n_test_cases = self.test_cases.borrow().len();

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
        self.menu.set(menu);
        if menu == Menu::Solve {
            self.reset_test_cases()
        }
    }

    pub fn reset_test_cases(&self) {
        let mut test_cases = self.test_cases.borrow_mut();
        let n = self.current_problem().test_cases.len();
        *test_cases = vec![TestCaseStatus::default(); n]
    }

    pub fn cancel_editing_field(&self) {
        self.input_handler.finish();
    }

    pub fn cancel_run(&self) {
        self.code_runner.please_stop().unwrap();
    }

    pub fn run_all_test_cases(&self) -> Result<(), SendError<RunRequest>> {
        let compile_script = self.settings.borrow().compilation_step.clone();
        let run_script = self.settings.borrow().run_step.clone();
        let test_cases = self.current_problem().test_cases.clone();

        self.code_runner
            .please_run(test_cases, compile_script, run_script)
    }

    pub fn update_test_cases(&self) -> bool {
        let updates = self.code_runner.get_updates();
        if updates.len() == 0 {
            return false;
        };

        self.new_test_cases_arrived.set(true);

        let mut test_cases = self.test_cases.borrow_mut();
        for RunResponse { id, status } in updates.into_iter() {
            let to_edit = test_cases
                .get_mut(id)
                .unwrap_or_else(|| panic!("Could not apply update run details for example {id}"));
            *to_edit = status;
        }

        true
    }

    pub fn current_problem<'a>(&'a self) -> Rc<Problem> {
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

    pub fn load(path: &String) -> Db {
        if Self::is_web_link(path) {
            Self::load_from_web(path)
        } else {
            Self::load_from_file(path)
        }
    }

    fn is_web_link(text: &String) -> bool {
        text.starts_with("http")
    }

    fn load_from_web(link: &String) -> Db {
        let response = minreq::get(link).send().unwrap();
        let serialized = response.as_str().unwrap();
        let owned: Vec<Problem> = serde_yaml::from_str(serialized).unwrap();
        owned.into_iter().map(|x| Rc::new(x)).collect()
    }

    fn load_from_file(path: &String) -> Db {
        let serialized =
            std::fs::read_to_string(path).expect("Something went wrong reading the file");
        let problems: Vec<Problem> = serde_yaml::from_str(&serialized).unwrap();
        problems.into_iter().map(Rc::new).collect()
    }

    pub fn finish_edit(&self) {
        let mut settings = self.settings.borrow_mut();
        let field = self.input_handler.current_field();
        let finished_input = self.input_handler.finish();
        match field {
            Some(field) => match field {
                InputField::CompileCommand => settings.compilation_step = finished_input,
                InputField::RunCommand => settings.run_step = finished_input,
            },
            None => {}
        }
    }

    pub fn get_test_cases(&self) -> Vec<TestCaseStatus> {
        self.update_test_cases();
        self.test_cases.borrow().clone()
    }

    pub fn details_for_selected_test_case(&self) -> (usize, TestCaseStatus) {
        let selected_id = self.selected_test_case.get();
        (
            selected_id,
            self.test_cases.borrow().get(selected_id).unwrap().clone(),
        )
    }

    pub fn check_for_changes(&self) -> bool {
        self.update_test_cases();
        let changes = self.new_test_cases_arrived.get();
        self.new_test_cases_arrived.set(false);
        changes
    }

    pub fn health_check(&self) {
        self.code_runner.health_check();
    }
}
