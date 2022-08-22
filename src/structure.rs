use crate::arguments::*;
use crate::structure::controller::Controller;
use crate::structure::controller::MainController;
use crate::structure::view::View;
use std::rc::Rc;
use tui::widgets::ListState;

pub mod common;
pub mod controller;
pub mod model;
mod runner;
pub mod settings;
pub mod ui;
pub mod view;

use common::*;
use model::Model;
use settings::Settings;

pub type ModelRef = Rc<Model>;

pub struct AppState {
    pub view: Rc<View>,
    pub controller: MainController,
}

impl Default for AppState {
    fn default() -> Self {
        let settings = Settings::default();
        let model: ModelRef = Model::new_ref(settings);
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

        let model: ModelRef = Model::new_ref(settings);
        let controller = MainController::setup(&model);
        let view = View::from(&model);

        Self {
            view: Rc::new(view),
            controller: controller,
        }
    }
}
