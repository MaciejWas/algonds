use crate::application::InputField;
use std::cell::Cell;
use std::cell::RefCell;

pub struct InputHandler {
    raw_input: RefCell<String>,
    direction: Cell<Option<InputField>>,
}
impl InputHandler {
    pub fn is_in_input_mode(&self) -> bool {
        self.direction.get().is_some()
    }

    pub fn current_field(&self) -> Option<InputField> {
        self.direction.get()
    }

    pub fn curr_input(&self) -> String {
        self.raw_input.borrow().clone()
    }

    pub fn edit_field(&self, curr_value: String, field: InputField) {
        self.direction.set(Some(field));
        self.raw_input.replace(curr_value);
    }

    pub fn add(&self, c: char) {
        if !self.is_in_input_mode() {
            panic!("inputting without")
        }
        self.raw_input.borrow_mut().push(c);
    }

    pub fn pop(&self) {
        if !self.is_in_input_mode() {
            panic!("inputting without")
        }
        self.raw_input.borrow_mut().pop();
    }

    pub fn finish(&self) -> String {
        self.direction.set(None);
        self.raw_input.replace(String::new())
    }
}
impl Default for InputHandler {
    fn default() -> Self {
        Self {
            raw_input: RefCell::default(),
            direction: Cell::default(),
        }
    }
}
