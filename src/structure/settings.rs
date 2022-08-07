use crate::AppArgs;
use serde::{Deserialize, Serialize};

const DB_ADDR: &str = "fsdafsadfs";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub db_path: String,
    pub solution_path: String,
    pub pretty: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            db_path: DB_ADDR.to_string(),
            solution_path: "./solution.py".to_string(),
            pretty: true,
        }
    }
}

impl From<&AppArgs> for Settings {
    fn from(args: &AppArgs) -> Self {
        let mut settings = Self::default();

        args.db_path
            .as_ref()
            .map(|path| settings.db_path = path.clone());
        settings.pretty = !args.disable_unicode;

        settings
    }
}

pub fn load(path: &String) -> Settings {
    if is_web_link(path) {
        load_from_web(path)
    } else {
        load_from_file(path)
    }
}

fn is_web_link(text: &String) -> bool {
    text.starts_with("http")
}

fn load_from_web(link: &String) -> Settings {
    let response = minreq::get(link).send().unwrap();
    let serialized = response.as_str().unwrap();
    serde_yaml::from_str(serialized).unwrap()
}

fn load_from_file(path: &String) -> Settings {
    todo!()
}
