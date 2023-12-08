//! ClassRef is the reference to the Class
//!
//! The class was initialized with `None`.
//!
//! Use `resolved_class` to resolve a class from ConstantClassInfo defined the class file.
//!
use crate::classfile::constant_pool::class::ConstantClassInfo;
use crate::classfile::constant_pool::consts::CONSTANT_CLASS;
use crate::rtda::heap::class::Class;
use crate::rtda::heap::constant::Constant;
use crate::rtda::heap::errors::RuntimeHeapError;
use crate::types::{OptionRcRefCell, RcRefCell};

#[derive(Clone)]
pub struct ClassRef {
    class_name: String,
    class: OptionRcRefCell<Class>,
}

impl ClassRef {
    pub fn new(class_info: &ConstantClassInfo) -> Self {
        ClassRef {
            class_name: class_info.name(),
            class: None,
        }
    }

    pub fn resolved_class(&mut self, class: RcRefCell<Class>) -> RcRefCell<Class> {
        if self.class.is_none() {
            self.resolve_class_ref(class);
        }
        self.class.clone().unwrap()
    }

    /// jvms8 5.4.3.1
    fn resolve_class_ref(&mut self, class: RcRefCell<Class>) {
        let loader = class.borrow_mut().loader().unwrap();
        let c = loader
            .borrow_mut()
            .load_class(loader.clone(), self.class_name.clone());

        if !c.borrow().is_accessible_to(&class.borrow()) {
            panic!("{}", RuntimeHeapError::IllegalAccess);
        }

        self.class = Some(c);
    }
}

impl Constant for ClassRef {
    fn tag(&self) -> u8 {
        CONSTANT_CLASS
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
