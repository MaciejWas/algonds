use std::sync::mpsc::SendError;
use tui::layout::Direction;
use std::ops::DerefMut;
use tui::widgets::ListState;
use crate::structure::common::*;
use crate::structure::runner::CodeRunner;
use crate::structure::common::Menu;
use crate::structure::Settings;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

pub type Db = Vec<Rc<Problem>>;

pub struct InputHandler {
    raw_input: RefCell<String>,
    direction: Cell<Option<InputField>>
} impl InputHandler {
    pub fn is_in_input_mode(&self) -> bool {
        self.direction.get().is_some()
    }

    pub fn current_field(&self) -> Option<InputField> {
        self.direction.get()
    }

    pub fn edit_field(&self, field: InputField) {
        self.direction.set(Some(field))
    }

    pub fn add(&self, c: char) {
        if !self.is_in_input_mode() {
            panic!("inputting without")
        }
        self.raw_input.borrow_mut().push(c);
    }

    pub fn pop(&self) {
        if !self.is_in_input_mode() {
            panic!("inputting without")
        }
        self.raw_input.borrow_mut().pop();
    }

    fn finish(&self) -> String {
        self.direction.set(None);
        self.raw_input.replace(String::new())
    }
}

impl Default for InputHandler {
    fn default() -> Self { 
        Self { raw_input: RefCell::default(), direction: Cell::default() } 
    }
}

pub struct Model {
    db: Db,
    pub input_handler: InputHandler,
    code_runner: CodeRunner,
    pub settings: RefCell<Settings>,
    pub menu: Cell<Menu>,
    pub test_cases: RefCell<Vec<ExampleStatus>>,
    pub list_state: RefCell<ListState>,
}

impl Model {
    pub fn new_ref(settings: Settings) -> Rc<Self> {
        Rc::new(Model {
            db: Model::load(&settings.db_path),
            code_runner: CodeRunner::default(),
            input_handler: InputHandler::default(),
            settings: RefCell::new(settings.clone()),
            menu: Cell::default(),
            test_cases: RefCell::default(),
            list_state: RefCell::default()
        })
    }

    pub fn select_next(&self, up: bool) {
        let id = self.list_state.borrow().selected();
        let change: i16 = if up { 1 } else { -1 };
        let new_id = id
            .map(|i| i as i16 + change)
            .map(|i| if i < 0 { 0 } else if i >= self.total_problems() as i16 { i - 1 } else { i } )
            .map(|i| i as usize)
            .or(Some(0));

        self.list_state.borrow_mut().select(new_id);
    }

    pub fn setup_problem(&self) {
        let n_tests = self.current_problem().examples.len();
        let mut test_cases = self.test_cases.borrow_mut();
        *test_cases = vec![ExampleStatus::default(); n_tests];
    }

    pub fn teardown_curr_problem(&self) {
        let mut test_cases = self.test_cases.borrow_mut();
        *test_cases = Vec::new();
    }

    pub fn run_test_cases(&self) -> Result<(), SendError<RunRequest>> {
        let compile_script = self.settings.borrow().compilation_step.clone();
        let run_script = self.settings.borrow().run_step.clone();

        self.code_runner.please_run(
            self.current_problem().examples.clone(),
            compile_script,
            run_script,
        )
    }

    pub fn stop_run_test_cases(&self) -> Result<(), SendError<RunRequest>> {
        self.code_runner.please_stop()
    }

    fn get_run_updates(&self) -> Vec<RunResponse> {
        self.code_runner.get_updates()
    }

    pub fn current_problem<'a>(&'a self) -> Rc<Problem> {
        let id: usize = self.list_state.borrow().selected().unwrap_or(0);
        self.db.get(id).unwrap().clone()
    }

    pub fn get_problems_in_range<'a>(
        &'a self,
        start_incl: usize,
        end_excl: usize,
    ) -> Vec<Rc<Problem>> {
        self.db
            .iter()
            .skip(start_incl)
            .take(end_excl - start_incl)
            .map(Rc::clone)
            .collect()
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

    pub fn add_to_input(&self, c: char) {
        self.input_handler.add(c);
    }

    pub fn finish_edit(&self) {
        let mut settings = self.settings.borrow_mut();
        let finished_input = self.input_handler.finish();
        match self.input_handler.current_field() {
            Some(field) => match field {
                InputField::CompileCommand => settings.compilation_step = finished_input,
                InputField::RunCommand => settings.run_step = finished_input,
            },
            None => {}
        }
    }

    pub fn test_cases() {

    }
}
