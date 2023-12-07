use crate::rtda::heap::object_ref::HeapObjectRefs;
use crate::types::RcRefCell;

use super::heap::class::Class;

pub static OBJECT_TAG: u8 = 10;

pub trait ObjectData {
    fn tag(&self) -> u8 {
        /// Default tag value for Object
        OBJECT_TAG
    }

    fn as_any(&self) -> &dyn std::any::Any;

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// ObjectExtra stores extra information about an object
///
/// Currently we use it to store the corresponding class object for this object
pub trait ObjectExtra {
    fn as_any(&self) -> &dyn std::any::Any;

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub struct Object {
    class: RcRefCell<Class>,
    data: Box<dyn ObjectData>,
    // Ref for Object, []int32 for int[] ...
    extra: Option<Box<dyn ObjectExtra>>,
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
        Object {
            class,
            data,
            extra: None,
        }
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

    pub fn extra(&self) -> Option<&dyn ObjectExtra> {
        self.extra.as_deref()
    }

    pub fn set_extra(&mut self, extra: Option<Box<dyn ObjectExtra>>) {
        self.extra = extra;
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

/// ClassData stores the corresponding class object for this object
pub struct ClassData {
    class: RcRefCell<Class>,
}

impl ClassData {
    pub fn new(class: RcRefCell<Class>) -> Self {
        ClassData { class }
    }

    pub fn java_name(&self) -> String {
        self.class.borrow().name().replace('/', ".")
    }
}

impl ObjectExtra for ClassData {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
