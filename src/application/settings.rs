use crate::AppArgs;
use serde::{Deserialize, Serialize};

const DB_ADDR: &str = "https://raw.githubusercontent.com/MaciejWas/algonds/main/src/data/db.yaml";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
            compilation_step: "echo hi there".to_string(),
            run_step: "echo hi there".to_string(),
            pretty: true,
        }
    }
}

impl From<&AppArgs> for Settings {
    fn from(args: &AppArgs) -> Self {
        let mut settings = Self::default();

        if let Some(db_path) = &args.db_path {
            settings.db_path = db_path.clone()
        }

        if let Some(comp_step) = &args.compilation_step {
            settings.compilation_step = comp_step.clone()
        }

        if let Some(run_step) = &args.run_step {
            settings.run_step = run_step.clone()
        }

        settings.pretty = !args.disable_unicode;
        settings
    }
}

impl Settings {
    pub fn load(path: &str) -> Settings {
        if is_web_link(path) {
            Self::load_from_web(path)
        } else {
            Self::load_from_file(path)
        }
    }

    fn load_from_web(link: &str) -> Settings {
        let response = minreq::get(link).send().unwrap();
        let serialized = response.as_str().unwrap();
        serde_yaml::from_str(serialized).unwrap()
    }

    fn load_from_file(path: &str) -> Settings {
        let serialized = std::fs::read_to_string(path).unwrap();
        serde_yaml::from_str(&serialized).unwrap()
    }
}

fn is_web_link(text: &str) -> bool {
    text.starts_with("http")
}
