#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::{init_class, invoke_method, Instruction};
use crate::instructions::errors::InstructionError;
use crate::rtda::frame::Frame;
use crate::rtda::heap::method_ref::MethodRef;

/// Invoke a class (static) method
#[derive(Default, Debug)]
pub struct INVOKE_STATIC {
    index: u64,
}

impl Instruction for INVOKE_STATIC {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_class = frame.method().borrow().get_class();
        let r_cp = current_class.borrow().constant_pool();
        let resolved_method = r_cp
            .borrow_mut()
            .get_constant_mut(self.index as usize)
            .as_any_mut()
            .downcast_mut::<MethodRef>()
            .unwrap()
            .resolved_method(current_class);
        if !resolved_method.borrow().is_static() {
            panic!("{}", InstructionError::IncompatibleClassChangeError);
        }

        let class = resolved_method.borrow().get_class();
        if !class.borrow().init_started() {
            frame.revert_next_pc();
            init_class(&frame.thread(), &class);
            return;
        }

        invoke_method(frame, &resolved_method);
    }
}
