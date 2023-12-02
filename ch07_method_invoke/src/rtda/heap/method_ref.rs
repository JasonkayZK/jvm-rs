use crate::classfile::constant_pool::consts::CONSTANT_METHOD_REF;
use crate::classfile::constant_pool::member_ref::ConstantMethodRefInfo;
use crate::rtda::heap::class::Class;
use crate::rtda::heap::constant::Constant;
use crate::rtda::heap::method::Method;
use crate::types::{OptionRcRefCell, RcRefCell};

pub struct MethodRef {
    class_name: String,
    class: OptionRcRefCell<Class>,
    name: String,
    descriptor: String,
    method: OptionRcRefCell<Method>,
}

impl MethodRef {
    pub fn new(ref_info: &ConstantMethodRefInfo) -> Self {
        let (name, descriptor) = ref_info.name_and_descriptor();
        MethodRef {
            class_name: ref_info.class_name(),
            class: None,
            name,
            descriptor,
            method: None,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn descriptor(&self) -> String {
        self.descriptor.clone()
    }

    pub fn resolved_method(&mut self) -> RcRefCell<Method> {
        if self.method.is_none() {
            self.resolve_method_ref();
        }
        self.method.clone().unwrap()
    }

    /// jvms8 5.4.3.3
    fn resolve_method_ref(&self) {
        // todo
        unimplemented!()
    }
}

impl Constant for MethodRef {
    fn tag(&self) -> u8 {
        CONSTANT_METHOD_REF
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
