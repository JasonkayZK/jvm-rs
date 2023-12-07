#![allow(non_camel_case_types)]

use crate::instructions::base::Instruction;
use crate::instructions::errors::InstructionError;
use crate::native::registry::NativeRegistry;
use crate::rtda::frame::Frame;

/// Invoke native method
///
/// Using `0xfe` reserved instruction
#[derive(Default, Debug)]
pub struct INVOKE_NATIVE;

impl Instruction for INVOKE_NATIVE {
    fn execute(&mut self, frame: &mut Frame) {
        let method = frame.method();
        let method_class = method.borrow().get_class();
        let class = method_class.borrow();
        let class_name = class.name();
        let method_name = method.borrow().name();
        let method_descriptor = method.borrow().descriptor();

        let native_method =
            NativeRegistry::find_native_method(class_name, &method_name, &method_descriptor);
        if native_method.is_none() {
            let method_info = format!("{}.{}{}", class_name, method_name, method_descriptor);
            panic!("{}", InstructionError::UnsatisfiedLinkError(method_info));
        }
        native_method.unwrap()(frame);
    }
}
