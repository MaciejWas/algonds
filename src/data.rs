use std::rc::Rc;
use crate::application::common::Problem;
use crate::application::model::Db;
use crate::application::common::TestCase;

pub fn load(path: &String) -> Db {
    if is_web_link(path) {
        load_from_web(path)
    } else {
        load_from_file(path)
    }
}

fn is_web_link(text: &String) -> bool {
    text.starts_with("http")
}

fn load_from_web(link: &String) -> Db {
    let response = minreq::get(link).send().unwrap();
    let serialized = response.as_str().unwrap();
    let owned: Vec<Problem> = serde_yaml::from_str(serialized).unwrap();
    owned.into_iter().map(Rc::new).collect()
}

fn load_from_file(path: &String) -> Db {
    let serialized =
        std::fs::read_to_string(path).expect("Something went wrong reading the file");
    let problems: Vec<Problem> = serde_yaml::from_str(&serialized).unwrap();
    problems.into_iter().map(Rc::new).collect()
}


pub fn generate_stress_tests_for(problem_name: &String, last_id: usize) -> Vec<TestCase> {
    match problem_name.as_str() {
        "Print \"Hello, world!\" (tutorial)" => stress_tests_for_hello_world(last_id),
        "Longest common substring" => stress_tests_for_lcs(last_id),
        _ => vec![]
    }
}

fn stress_tests_for_lcs(last_id: usize) -> Vec<TestCase> {
    vec![
        TestCase {
            id: last_id + 1,
            complexity: 100 * 6,
            input: "abcde".repeat(100) + " " + &"abcdez".repeat(100),
            output: "abcde".to_string(),
            is_stress_test: true,
        },
        TestCase {
            id: last_id + 2,
            complexity: 1000 * 6,
            input: "abcde".repeat(1000) + " " + &"abcdez".repeat(1000),
            output: "abcde".repeat(230),
            is_stress_test: true,
        },
        TestCase {
            id: last_id + 3,
            complexity: 10_000 * 6,
            input: "abcde".repeat(10_000) + " " + &"abcdez".repeat(10_000),
            output: "abcde".to_string(),
            is_stress_test: true,
        },
    ]
}

fn stress_tests_for_hello_world(last_id: usize) -> Vec<TestCase> {
    vec![
        TestCase {
            id: last_id + 1,
            complexity: 100,
            input: "a".repeat(100),
            output: "Hello, world!".to_string(),
            is_stress_test: true,
        },
        TestCase {
            id: last_id + 2,
            complexity: 1000,
            input: "a".repeat(1000),
            output: "Hello, world!".to_string(),
            is_stress_test: true,
        },
        TestCase {
            id: last_id + 3,
            complexity: 10000,
            input: "a".repeat(10000),
            output: "Hello, world!".to_string(),
            is_stress_test: true,
        }
    ]
}