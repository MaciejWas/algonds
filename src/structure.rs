use tui::widgets::ListState;
use crate::arguments::*;
use crate::structure::controller::Controller;
use crate::structure::view::View;
use std::rc::Rc;

pub mod common;
pub mod controller;
pub mod model;
mod runner;
pub mod settings;
pub mod view;
pub mod ui;

use common::*;
use model::Model;
use settings::Settings;

pub type ModelRef = Rc<Model>;

pub struct AppState {
    pub view: Rc<View>,
    pub controller: Controller,
}

impl Default for AppState {
    fn default() -> Self { 
        let settings = Settings::default();
        let model: ModelRef = Model::new_ref(settings);
            let controller = Controller::from(&model);
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
        let controller = Controller::from(&model);
        let view = View::from(&model);

        Self {
            view: Rc::new(view),
            controller: controller,
        }
    }
}
