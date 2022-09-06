use crate::data::generate_stress_tests_for;
use std::rc::Rc;
use crate::application::Problem;
use crate::application::TestCase;
use crate::application::RunResponse;
use std::cell::RefCell;
use crate::application::test_runner::CodeRunner;
use crate::application::TestCaseStatus;
use std::cell::Cell;

pub struct TestSuite {
    new_test_cases_arrived: Cell<bool>,
    test_cases: RefCell<Vec<TestCase>>,
    test_case_statuses: RefCell<Vec<TestCaseStatus>>,
    code_runner: CodeRunner, 
}

impl TestSuite {
    pub fn new() -> Self {
        Self {
            new_test_cases_arrived: Cell::default(),
            test_cases: RefCell::default(),
            test_case_statuses: RefCell::default(),
            code_runner: CodeRunner::default(),
        }
    }

    pub fn number_of_tests(&self) -> usize {
        self.test_cases.borrow().len()
    }


    pub fn update_test_cases(&self) -> bool {
        let updates = self.code_runner.get_updates();
        if updates.len() == 0 {
            return false;
        };

        self.new_test_cases_arrived.set(true);

        let mut test_cases = self.test_case_statuses.borrow_mut();
        for RunResponse { id, status } in updates.into_iter() {
            let to_edit = test_cases
                .get_mut(id)
                .unwrap_or_else(|| panic!("Could not apply update run details for example {id}"));
            *to_edit = status;
        }

        true
    }

    pub fn get_test_cases(&self) -> Vec<TestCaseStatus> {
        self.update_test_cases();
        self.test_case_statuses.borrow().clone()
    }

    pub fn get_nth_test_case(&self, n: usize) -> TestCaseStatus {
        self.test_case_statuses.borrow().get(n).unwrap().clone()
    }

    pub fn check_for_changes(&self) -> bool {
        self.update_test_cases();
        let changes = self.new_test_cases_arrived.get();
        self.new_test_cases_arrived.set(false);
        changes
    }

    pub fn run(&self, compile_script: String, run_script: String) {
        self.reset_test_cases();
        self.code_runner
            .please_run(self.test_cases.borrow().clone(), compile_script, run_script)
            .unwrap();
    }

    pub fn stop(&self) {
        self.code_runner.please_stop().unwrap();
    }

    pub fn set_test_cases_from(&self, problem: Rc<Problem>) {
        let mut test_cases = problem.test_cases.clone();
        let last_id = test_cases.iter().map(|tc| tc.id).max().unwrap_or(0);
        let mut stress_tests = generate_stress_tests_for(&problem.name, last_id);
        test_cases.append(&mut stress_tests);
    
        let n = test_cases.len();
        *self.test_cases.borrow_mut() = test_cases;
        *self.test_case_statuses.borrow_mut() = vec![TestCaseStatus::default(); n];
    }

    fn reset_test_cases(&self) {
        let n = self.number_of_tests();
        *self.test_case_statuses.borrow_mut() = vec![TestCaseStatus::default(); n]
    }
}
