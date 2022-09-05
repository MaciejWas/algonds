
pub fn generate_test_cases_for(id: usize) -> [TestCase; 3] {
    match id {
        0 => {}
        1 => {}
        2 => {}
        3 => {}
        _ => unreachable!
    }
}

pub trait StressTestGen {
    fn for_problem_with_id(id: usize);
    fn generate(&self) -> TestCase;
}