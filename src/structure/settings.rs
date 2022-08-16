use crate::AppArgs;
use serde::{Deserialize, Serialize};

const DB_ADDR: &str = "https://raw.githubusercontent.com/MaciejWas/algonds/main/src/data/db.yaml";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub db_path: String,
    pub compilation_step: String,
    pub run_step: String,
    pub pretty: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            db_path: DB_ADDR.to_string(),
            compilation_step: "".to_string(),
            run_step: "python ./solution.py {args}".to_string(),
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
        args.compilation_step
            .as_ref()
            .map(|txt| settings.compilation_step = txt.clone());
        args.run_step
            .as_ref()
            .map(|txt| settings.run_step = txt.clone());

        settings.pretty = !args.disable_unicode;

        settings
    }
}

impl Settings {
    pub fn load(path: &String) -> Settings {
        if is_web_link(path) {
            Self::load_from_web(path)
        } else {
            Self::load_from_file(path)
        }
    }

    fn load_from_web(link: &String) -> Settings {
        let response = minreq::get(link).send().unwrap();
        let serialized = response.as_str().unwrap();
        serde_yaml::from_str(serialized).unwrap()
    }

    fn load_from_file(path: &String) -> Settings {
        todo!()
    }
}

fn is_web_link(text: &String) -> bool {
    text.starts_with("http")
}
