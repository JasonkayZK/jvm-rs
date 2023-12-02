use crate::rtda::heap::object_ref::HeapObjectRefs;
use crate::types::RcRefCell;

use super::heap::class::Class;

#[derive(Clone)]
pub struct Object {
    class: RcRefCell<Class>,
    fields: HeapObjectRefs,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        let _self = self as *const Self;
        let _other = other as *const Self;
        _self.eq(&_other)
    }
}

impl Object {
    pub fn new(class: RcRefCell<Class>) -> Self {
        Object {
            class: class.clone(),
            fields: HeapObjectRefs::new(class.borrow().instance_object_ref_count() as usize),
        }
    }

    pub fn class(&self) -> &RcRefCell<Class> {
        &self.class
    }

    pub fn fields(&self) -> &HeapObjectRefs {
        &self.fields
    }

    pub fn fields_mut(&mut self) -> &mut HeapObjectRefs {
        &mut self.fields
    }

    pub fn is_instance_of(&self, class: &RcRefCell<Class>) -> bool {
        class.borrow().is_assignable_from(class, &self.class)
    }
}
