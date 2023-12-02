use crate::classfile::constant_pool::consts::CONSTANT_FIELD_REF;
use crate::classfile::constant_pool::member_ref::ConstantFieldRefInfo;
use crate::rtda::heap::constant::Constant;
use crate::rtda::heap::errors::RuntimeHeapError;
use crate::types::{OptionRcRefCell, RcRefCell};

use super::class::Class;
use super::field::Field;

pub struct FieldRef {
    class_name: String,
    class: OptionRcRefCell<Class>,
    name: String,
    descriptor: String,
    field: OptionRcRefCell<Field>,
}

impl FieldRef {
    pub fn new(ref_info: &ConstantFieldRefInfo) -> Self {
        let (name, descriptor) = ref_info.name_and_descriptor();
        FieldRef {
            class_name: ref_info.class_name(),
            class: None,
            name,
            descriptor,
            field: None,
        }
    }

    pub fn resolved_field(&mut self, class: RcRefCell<Class>) -> RcRefCell<Field> {
        if self.field.is_none() {
            self.resolve_field_ref(class);
        }
        self.field.clone().unwrap()
    }

    /// jvms 5.4.3.2
    fn resolve_field_ref(&mut self, class: RcRefCell<Class>) {
        let c = self.resolved_class(class.clone());
        let field = Self::lookup_field(&c, self.name.clone(), self.descriptor.clone());

        if field.is_none() {
            panic!("{}", RuntimeHeapError::NoSuchField);
        }

        if !field.as_ref().unwrap().borrow().is_accessible_to(&class) {
            panic!("{}", RuntimeHeapError::IllegalAccess);
        }

        self.field = field;
    }

    fn lookup_field(
        class: &RcRefCell<Class>,
        name: String,
        descriptor: String,
    ) -> OptionRcRefCell<Field> {
        for field in class.borrow_mut().fields() {
            if field.borrow().name() == name.clone()
                && field.borrow().descriptor() == descriptor.clone()
            {
                return Some(field);
            }
        }

        // Search from interfaces
        if let Some(interfaces) = class.borrow_mut().interfaces() {
            for iface in interfaces {
                let field = Self::lookup_field(&iface, name.clone(), descriptor.clone());
                if field.is_some() {
                    return field;
                }
            }
        }

        // Search from super
        if let Some(super_class) = class.borrow().super_class() {
            return Self::lookup_field(&super_class, name, descriptor);
        }

        None
    }

    fn resolved_class(&mut self, class: RcRefCell<Class>) -> RcRefCell<Class> {
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

impl Constant for FieldRef {
    fn tag(&self) -> u8 {
        CONSTANT_FIELD_REF
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
