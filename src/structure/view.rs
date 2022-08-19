use tui::widgets::ListState;
use crate::structure::common::*;
use crate::structure::ModelRef;
use crate::structure::Problem;
use std::cell::RefCell;
use std::rc::Rc;

pub struct View {
    model: ModelRef,
}

impl From<&ModelRef> for View {
    fn from(model: &ModelRef) -> Self {
        Self {
            model: Rc::clone(model),
        }
    }
}

impl View {
    pub fn get_list_state(&self) -> RefCell<ListState> {
        self.model.list_state.clone()
    }

    pub fn get_problems_to_select<'a>(&self) -> Vec<Rc<Problem>> {
        self.model.get_problems_in_range(0, usize::MAX)
    }

    pub fn get_cursor(&self) -> String {
        let sign = if self.model.settings.borrow().pretty {
           " 🡆 "
        } else { 
            " -> "
        };
        sign.to_string()
    }

    pub fn curr_menu(&self) -> Menu {
        self.model.menu.get()
    }

    pub fn current_problem<'a>(&self) -> Rc<Problem> {
        self.model.current_problem()
    }
}
