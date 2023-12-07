use std::cell::RefCell;
use std::rc::Rc;

use crate::classfile::member_info::MemberInfo;
use crate::rtda::heap::access_flags::{
    ACC_ENUM, ACC_FINAL, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC, ACC_SYNTHETIC,
    ACC_TRANSIENT, ACC_VOLATILE,
};
use crate::rtda::heap::class::Class;
use crate::types::{OptionRcRefCell, RcRefCell, VecRcRefCell};

#[derive(Default)]
pub struct Field {
    access_flags: u16,
    name: String,
    descriptor: String,
    class: OptionRcRefCell<Class>,

    /// 常量池中的索引
    const_value_index: u64,
    /// 字段在 ObjectRefs 中的索引
    object_ref_id: u64,
}

impl Field {
    pub fn new_fields(class: RcRefCell<Class>, cf_fields: &Vec<MemberInfo>) -> VecRcRefCell<Self> {
        let mut fields = Vec::new();
        for m in cf_fields {
            fields.push(Self::new(class.clone(), m));
        }
        fields
    }

    pub fn is_public(&self) -> bool {
        self.access_flags & ACC_PUBLIC != 0
    }

    pub fn is_private(&self) -> bool {
        self.access_flags & ACC_PRIVATE != 0
    }

    pub fn is_protected(&self) -> bool {
        self.access_flags & ACC_PROTECTED != 0
    }

    pub fn is_static(&self) -> bool {
        self.access_flags & ACC_STATIC != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags & ACC_FINAL != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.access_flags & ACC_SYNTHETIC != 0
    }

    pub fn is_volatile(&self) -> bool {
        self.access_flags & ACC_VOLATILE != 0
    }

    pub fn is_transient(&self) -> bool {
        self.access_flags & ACC_TRANSIENT != 0
    }

    pub fn is_enum(&self) -> bool {
        self.access_flags & ACC_ENUM != 0
    }

    pub fn is_long_or_double(&self) -> bool {
        self.descriptor == "J" || self.descriptor == "D"
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn descriptor(&self) -> &str {
        &self.descriptor
    }

    pub fn get_class(&self) -> OptionRcRefCell<Class> {
        self.class.clone()
    }

    pub fn const_value_index(&self) -> u64 {
        self.const_value_index
    }

    pub fn set_object_ref_id(&mut self, object_ref_id: u64) {
        self.object_ref_id = object_ref_id;
    }

    pub fn object_ref_id(&self) -> u64 {
        self.object_ref_id
    }

    /// jvms 5.4.4
    pub fn is_accessible_to(&self, class: &RcRefCell<Class>) -> bool {
        if self.is_public() {
            return true;
        }
        let c = self.class.as_ref().unwrap();
        if self.is_protected() {
            return class.eq(c)
                || class.borrow().is_sub_class_of(c)
                || c.borrow().get_package_name() == class.borrow().get_package_name();
        }
        if !self.is_private() {
            return c.borrow().get_package_name() == class.borrow().get_package_name();
        }
        class.eq(c)
    }

    fn new(class: RcRefCell<Class>, m: &MemberInfo) -> RcRefCell<Self> {
        let const_value_index = match m.constant_value_attribute() {
            Some(val_attr) => val_attr.constant_value_index() as u64,
            None => 0,
        };

        let field = Field {
            access_flags: m.access_flags(),
            name: m.name(),
            descriptor: m.descriptor(),
            class: Some(class),
            // Currently has no reference, modified by set_object_ref_id
            object_ref_id: 0,
            const_value_index,
        };

        Rc::new(RefCell::new(field))
    }
}
