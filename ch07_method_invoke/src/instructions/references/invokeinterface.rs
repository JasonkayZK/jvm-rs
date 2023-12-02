#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::{invoke_method, Instruction};
use crate::instructions::errors::InstructionError;
use crate::rtda::frame::Frame;
use crate::rtda::heap::interface_ref::InterfaceMethodRef;
use crate::rtda::heap::method_lookup::lookup_method_in_class;

/// Invoke interface method
#[derive(Default, Debug)]
pub struct INVOKE_INTERFACE {
    index: u64,
    // count: u8,
    // zero: u8,
}

impl Instruction for INVOKE_INTERFACE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
        reader.read_u8(); // count
        reader.read_u8(); // must be 0
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_class = frame.method().borrow().get_class();
        let r_cp = current_class.borrow().constant_pool();
        let resolved_class = r_cp
            .borrow_mut()
            .get_constant_mut(self.index as usize)
            .as_any_mut()
            .downcast_mut::<InterfaceMethodRef>()
            .unwrap()
            .resolved_class(current_class);
        let resolved_method = r_cp
            .borrow_mut()
            .get_constant_mut(self.index as usize)
            .as_any_mut()
            .downcast_mut::<InterfaceMethodRef>()
            .unwrap()
            .resolved_interface_method(resolved_class.clone());

        if resolved_method.borrow().is_static() || resolved_method.borrow().is_private() {
            panic!("{}", InstructionError::IncompatibleClassChangeError);
        }

        let _ref = frame
            .operand_stack_mut()
            .get_ref_from_top((resolved_method.borrow().arg_object_ref_count() - 1) as usize);
        if _ref.is_none() {
            panic!("{}", InstructionError::NullPointerException);
        }
        if !_ref
            .as_ref()
            .unwrap()
            .borrow()
            .class()
            .borrow()
            .is_implements(&resolved_class)
        {
            panic!("{}", InstructionError::IncompatibleClassChangeError);
        }

        let method_to_be_invoked = lookup_method_in_class(
            _ref.as_ref().unwrap().borrow().class(),
            resolved_method.borrow().name(),
            resolved_method.borrow().descriptor(),
        );

        if method_to_be_invoked.is_none()
            || method_to_be_invoked
                .as_ref()
                .unwrap()
                .borrow()
                .is_abstract()
        {
            panic!("{}", InstructionError::AbstractMethodError);
        }
        if !method_to_be_invoked.as_ref().unwrap().borrow().is_public() {
            panic!("{}", InstructionError::IllegalAccessError);
        }

        invoke_method(frame, method_to_be_invoked.as_ref().unwrap());
    }
}
