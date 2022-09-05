use crate::arguments::*;
use controller::Controller;
use controller::MainController;
use std::rc::Rc;
use view::View;

pub mod common;
pub mod controller;
mod input_handler;
pub mod model;
pub mod settings;
mod test_runner;
pub mod ui;
pub mod view;

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
            controller: controller,
        }
    }
}

impl From<AppArgs> for AppState {
    fn from(args: AppArgs) -> Self {
        let settings = args
            .settings
            .as_ref()
            .map(|settings_path| Settings::load(&settings_path))
            .unwrap_or(Settings::from(&args));

        let model: Rc<Model> = Model::new_ref(settings);
        let controller = MainController::setup(&model);
        let view = View::from(&model);

        Self {
            view: Rc::new(view),
            controller: controller,
        }
    }
}
