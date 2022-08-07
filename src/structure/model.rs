use crate::structure::common::ExampleStatus;
use crate::structure::common::Example;
use std::cell::Cell;
use crate::structure::common::Problem;
use crate::structure::view::Menu;
use crate::structure::Settings;
use std::cell::RefCell;
use std::rc::Rc;

pub type Db = Vec<Rc<Problem>>;

pub struct Model {
    pub db: Db,
    pub settings: RefCell<Settings>,
    pub menu: Cell<Menu>,
    pub curr_prob_id: Cell<usize>,
}

impl Model {
    pub fn new_ref(settings: Settings) -> Rc<Self> {
        Rc::new(Model {
            db: Model::load(&settings.db_path),
            settings: RefCell::new(settings),
            menu: Cell::new(Menu::Select),
            curr_prob_id: Cell::new(0),
        })
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
        // let serialized = std::fs::read_to_string(path)
        // .expect("Something went wrong reading the file");
        // serde_yaml::from_str(&serialized).unwrap()
        let mut ps = Vec::new();
        for i in 0..40 {
            ps.push(Rc::new(Problem::default()));
        }
        ps
    }
}

struct Command {
    setup: String,
    run: String
}

impl Command {
    pub fn exec_script(&self, solution_path: &String, args: &String)  -> String {
        [
            self.setup.replace("{solution_path}", solution_path),
            self.run.replace("{args}", args)
        ].join(" && ")
    }
}


// struct CodeRunner {
//     examples: Vec<Example>,
//     solution_path: String, 
//     command: Command,   
// } impl CodeRunner {
//     fn run(&self) {
//         let script = self.command.exec_script(solution_path)
//     } 
// }

pub enum MessageToRunner {
    SetExamples(Vec<Example>),
    SetSolution(String),
    SetCommand(Command),
    Run,
    Abort
}

pub enum MessageFromRunner {
    Finished(usize, ExampleStatus)
}



