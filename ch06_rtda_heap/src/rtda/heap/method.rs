use std::cell::RefCell;
use std::rc::Rc;

use crate::classfile::member_info::MemberInfo;
use crate::rtda::heap::access_flags::{
    ACC_ABSTRACT, ACC_BRIDGE, ACC_FINAL, ACC_NATIVE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC,
    ACC_STATIC, ACC_STRICT, ACC_SYNCHRONIZED, ACC_SYNTHETIC, ACC_VARARGS,
};
use crate::rtda::heap::class::Class;
use crate::types::{OptionRcRefCell, RcRefCell};

#[derive(Default)]
pub struct Method {
    access_flags: u16,
    name: String,
    descriptor: String,
    class: OptionRcRefCell<Class>,

    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
}

impl Method {
    pub fn new_methods(
        class: RcRefCell<Class>,
        cf_methods: &Vec<MemberInfo>,
    ) -> Vec<RcRefCell<Method>> {
        let mut methods = Vec::new();
        for m in cf_methods {
            methods.push(Method::new(class.clone(), m));
        }
        methods
    }

    pub fn new(class: RcRefCell<Class>, cf_method: &MemberInfo) -> RcRefCell<Self> {
        let (max_stack, max_locals, code) = match cf_method.code_attribute() {
            Some(code_attr) => (
                code_attr.max_stack(),
                code_attr.max_locals(),
                code_attr.code(),
            ),
            None => (0, 0, vec![]),
        };

        Rc::new(RefCell::new(Method {
            access_flags: cf_method.access_flags(),
            name: cf_method.name(),
            descriptor: cf_method.descriptor(),
            class: Some(class),
            max_stack,
            max_locals,
            code,
        }))
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

    pub fn is_synchronized(&self) -> bool {
        self.access_flags & ACC_SYNCHRONIZED != 0
    }

    pub fn is_bridge(&self) -> bool {
        self.access_flags & ACC_BRIDGE != 0
    }

    pub fn is_varargs(&self) -> bool {
        self.access_flags & ACC_VARARGS != 0
    }

    pub fn is_native(&self) -> bool {
        self.access_flags & ACC_NATIVE != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.access_flags & ACC_ABSTRACT != 0
    }

    pub fn is_strict(&self) -> bool {
        self.access_flags & ACC_STRICT != 0
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn descriptor(&self) -> String {
        self.descriptor.clone()
    }

    pub fn get_class(&self) -> RcRefCell<Class> {
        self.class.clone().unwrap()
    }

    pub fn max_stack(&self) -> u16 {
        self.max_stack
    }

    pub fn max_locals(&self) -> u16 {
        self.max_locals
    }

    pub fn code(&self) -> Vec<u8> {
        self.code.clone()
    }
}
