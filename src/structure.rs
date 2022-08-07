use crate::arguments::*;
use crate::structure::controller::Controller;
use crate::structure::view::View;
use std::rc::Rc;

pub mod common;
pub mod controller;
pub mod model;
pub mod settings;
pub mod view;

use common::{Difficulty, Problem};
use model::Model;
use settings::Settings;

pub type ModelRef = Rc<Model>;

pub struct AppState {
    pub view: Rc<View>,
    pub controller: Controller,
    pub settings: Settings,
}

impl From<AppArgs> for AppState {
    fn from(args: AppArgs) -> Self {
        let settings = args
            .settings
            .as_ref()
            .map(|path| settings::load(&path))
            .unwrap_or(Settings::from(&args));

        let model: ModelRef = Model::new_ref(settings.clone());
        let controller = Controller::from(&model);
        let view = View::from(&model);

        Self {
            view: Rc::new(view),
            controller: controller,
            settings: settings,
        }
    }
}
