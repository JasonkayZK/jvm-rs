use crate::rtda::heap::object_ref::HeapObjectRefs;
use crate::types::RcRefCell;

use super::heap::class::Class;

pub trait ObjectData {
    fn tag(&self) -> u8 {
        10
    }

    fn as_any(&self) -> &dyn std::any::Any;

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub struct Object {
    class: RcRefCell<Class>,
    data: Box<dyn ObjectData>, // Ref for Object, []int32 for int[] ...
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        let _self = self as *const Self;
        let _other = other as *const Self;
        _self.eq(&_other)
    }
}

impl Object {
    /// Create normal (non-array) object
    pub fn new(class: RcRefCell<Class>) -> Self {
        Object::new_data(
            class.clone(),
            Box::new(HeapObjectRefs::new(
                class.borrow().instance_object_ref_count() as usize,
            )),
        )
    }

    pub fn new_data(class: RcRefCell<Class>, data: Box<dyn ObjectData>) -> Self {
        Object { class, data }
    }

    pub fn class(&self) -> &RcRefCell<Class> {
        &self.class
    }

    pub fn fields(&self) -> &HeapObjectRefs {
        self.data
            .as_any()
            .downcast_ref::<HeapObjectRefs>()
            .as_ref()
            .unwrap()
    }

    pub fn fields_mut(&mut self) -> &mut HeapObjectRefs {
        self.data
            .as_any_mut()
            .downcast_mut::<HeapObjectRefs>()
            .unwrap()
    }

    pub fn is_instance_of(&self, class: &RcRefCell<Class>) -> bool {
        class.borrow().is_assignable_from(class, &self.class)
    }

    pub fn data(&self) -> &dyn ObjectData {
        self.data.as_ref()
    }

    pub fn data_mut(&mut self) -> &mut dyn ObjectData {
        self.data.as_mut()
    }

    /// Reflection
    pub fn get_ref_var(&mut self, name: String, descriptor: String) -> RcRefCell<Object> {
        let field = self.class.borrow().get_field(name, descriptor, false);
        let slots = self
            .data
            .as_any_mut()
            .downcast_mut::<HeapObjectRefs>()
            .unwrap();
        slots
            .get_ref(field.unwrap().borrow().object_ref_id() as usize)
            .unwrap()
    }

    /// Reflection
    pub fn set_ref_var(&mut self, name: String, descriptor: String, _ref: RcRefCell<Object>) {
        let field = self.class.borrow().get_field(name, descriptor, false);
        let slots = self
            .data
            .as_any_mut()
            .downcast_mut::<HeapObjectRefs>()
            .unwrap();
        slots.set_ref(field.unwrap().borrow().object_ref_id() as usize, Some(_ref));
    }
}
