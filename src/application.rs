use crate::arguments::*;
use controller::{MainController, Controller};
use std::rc::Rc;
use view::View;

pub mod common;
pub mod controller;
pub mod model;
pub mod settings;
pub mod ui;
pub mod view;

mod test_suite;
mod test_runner;
mod input_handler;

use common::*;
use model::Model;
use settings::Settings;

pub struct AppState {
    pub view: Rc<View>,
    pub controller: MainController,
}

impl Default for AppState {
    fn default() -> Self {
        let settings = Settings::default();
        let model = Model::new_ref(settings);
        let controller = MainController::setup(&model);
        let view = View::from(&model);

        Self {
            view: Rc::new(view),
            controller,
        }
    }
}

impl From<AppArgs> for AppState {
    fn from(args: AppArgs) -> Self {
        let settings = args
            .settings
            .as_ref()
            .map(|settings_path| Settings::load(settings_path))
            .unwrap_or_else(|| Settings::from(&args));

        let model: Rc<Model> = Model::new_ref(settings);
        let controller = MainController::setup(&model);
        let view = View::from(&model);

        Self {
            view: Rc::new(view),
            controller,
        }
    }
}
