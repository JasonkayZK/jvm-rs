#![allow(clippy::if_same_then_else)]

use crate::rtda::heap::class_name_helper::get_component_class_name;
use crate::rtda::heap::errors::RuntimeHeapError;
use crate::rtda::object::Object;
use crate::types::{OptionRcRefCell, RcRefCell};

use super::class::Class;

impl Class {
    pub fn is_array(&self) -> bool {
        self.name().as_bytes()[0] == b'['
    }

    pub fn new_array(&self, self_ref: RcRefCell<Class>, count: usize) -> Object {
        if !self.is_array() {
            panic!(
                "{}",
                RuntimeHeapError::NotArrayClass(self.name().to_string())
            );
        }

        if self.name() == "[Z" || self.name() == "[B" {
            Object::new_data(self_ref, Box::new(vec![0_i8; count]))
        } else if self.name() == "[C" {
            Object::new_data(self_ref, Box::new(vec![0_u16; count]))
        } else if self.name() == "[S" {
            Object::new_data(self_ref, Box::new(vec![0_i16; count]))
        } else if self.name() == "[I" {
            Object::new_data(self_ref, Box::new(vec![0_i32; count]))
        } else if self.name() == "[J" {
            Object::new_data(self_ref, Box::new(vec![0_i64; count]))
        } else if self.name() == "[F" {
            Object::new_data(self_ref, Box::new(vec![0_f32; count]))
        } else if self.name() == "[D" {
            Object::new_data(self_ref, Box::new(vec![0_f64; count]))
        } else {
            Object::new_data(
                self_ref,
                Box::new(vec![None as OptionRcRefCell<Object>; count]),
            )
        }
    }

    pub fn component_class(&mut self) -> RcRefCell<Class> {
        let component_class_name = get_component_class_name(self.name());
        let mut loader = self.loader();
        let loader = loader.as_mut().unwrap();
        return loader
            .borrow_mut()
            .load_class(loader.clone(), component_class_name);
    }
}
