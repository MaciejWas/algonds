use crate::structure::model::RunRequest::PleaseRun;
use crate::structure::common::*;
use crate::structure::view::Menu;
use crate::structure::Settings;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

pub type Db = Vec<Rc<Problem>>;

pub struct Model {
    db: Db,
    directing_input_to: Cell<Option<InputField>>,
    pub input: RefCell<String>,
    pub settings: RefCell<Settings>,
    pub menu: Cell<Menu>,
    pub curr_prob_id: Cell<usize>,
}

impl Model {
    pub fn new_ref(settings: Settings) -> Rc<Self> {
        Rc::new(Model {
            db: Model::load(&settings.db_path),
            input: RefCell::default(),
            settings: RefCell::new(settings.clone()),
            menu: Cell::default(),
            curr_prob_id: Cell::default(),
            directing_input_to: Cell::new(None)
        })
    }

    pub fn direct_input_to(&self, field: InputField) {
        self.directing_input_to.set(Some(field));

        let mut input = self.input.borrow_mut();
        match field {
            InputField::CompileCommand => *input = self.settings.borrow().compilation_step.clone(),
            InputField::RunCommand => *input = self.settings.borrow().run_step.clone(),
        }        
    }

    pub fn current_problem<'a>(&'a self) -> &'a Problem {
        let id: usize = self.curr_prob_id.get();
        self.db.get(id).unwrap()
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
        let mut x = self.input.borrow_mut();
        x.push(c);
    }

    pub fn wipe_input(&self) {
        let mut x = self.input.borrow_mut();
        *x = String::new()
    }

    pub fn is_in_input_mode(&self) -> bool {
        self.directing_input_to.get().is_some()
    }

    pub fn additional_data(&self) -> AdditionalData {
        if let Some(field) = self.directing_input_to.get() {
            return match field {
                InputField::CompileCommand => AdditionalData::CompileCommand(self.input.borrow().clone()),
                InputField::RunCommand => AdditionalData::RunCommand(self.input.borrow().clone()),
            }
        }

        AdditionalData::None
    }

    pub fn save_input(&self) {
        let mut settings = self.settings.borrow_mut();
        let finished_input = self.input.borrow().clone();
        match self.directing_input_to.get() {
            Some(field) => match field {
                InputField::CompileCommand => settings.compilation_step = finished_input,
                InputField::RunCommand => settings.run_step = finished_input,
            }
            None => {}
        }
    }

    pub fn finish_input(&self) {
        self.directing_input_to.set(None);
    }
}

#[derive(Clone)]
enum RunRequest {
    PleaseRun {
        compile_script: String,
        run_script: String,
        examples: Vec<Example>
    },
    PleaseStop
}

struct RunResponse {
    id: usize,
    result: ExampleStatus
}

use std::sync::mpsc;
use std::thread;

struct CodeRunner {
    incoming: mpsc::Receiver<RunResponse>,
    outgoing:  mpsc::Sender<RunRequest>
} impl CodeRunner {
    pub fn new() -> Self {
        let (to_main, from_runner) = mpsc::channel();
        let (to_runner, from_main) = mpsc::channel();

        thread::spawn(move || {
            RemoteRunner::new(from_main, to_main).run();
        });

        Self { incoming: from_runner, outgoing: to_runner }
    }
}

struct RemoteRunner {
    incoming: mpsc::Receiver<RunRequest>,
    outgoing: mpsc::Sender<RunResponse>,
    last_run_reqest: Option<RunRequest>,
    running: usize,
}

impl RemoteRunner {
    pub fn new(incoming: mpsc::Receiver<RunRequest>, outgoing: mpsc::Sender<RunResponse>) -> Self {
        Self { incoming, outgoing, last_run_reqest: None, running: 0 }
    }

    pub fn run(mut self) {
        loop {
            self.recv();
            self.run_next_example();
        }    
    }

    fn should_stop(&self) -> bool {
        todo!()
    }

    fn run_next_example(&self) -> ExampleStatus {
        todo!()
    }

    fn send_result(&self, result: ExampleStatus, example_id: usize) {
        todo!()
    }

    fn recv(&mut self) {
        match self.incoming.recv_timeout(std::time::Duration::from_secs(1)) {
            Ok(request) => match request {
                RunRequest::PleaseRun { .. } => self.last_run_reqest = Some(request),
                RunRequest::PleaseStop       => self.reset()
            }
            Err(_) => panic!("this should never happen!"),
        }
    }

    fn reset(&mut self) {
        self.last_run_reqest = None;
        self.running = 0;
    }
}