#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::{init_class, Instruction};
use crate::instructions::errors::InstructionError;
use crate::rtda::frame::Frame;
use crate::rtda::heap::field_ref::FieldRef;

/// Get static field from class
#[derive(Default, Debug)]
pub struct GET_STATIC {
    index: u64,
}

impl Instruction for GET_STATIC {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_method = frame.method();
        let current_class = current_method.borrow().get_class();
        let r_cp = current_class.borrow().constant_pool();
        let field = r_cp
            .borrow_mut()
            .get_constant_mut(self.index as usize)
            .as_any_mut()
            .downcast_mut::<FieldRef>()
            .unwrap()
            .resolved_field(current_class);

        let class = field.borrow().get_class().unwrap();

        if !class.borrow().init_started() {
            frame.revert_next_pc();
            init_class(&frame.thread(), &class);
            return;
        }

        if !field.borrow().is_static() {
            panic!("{}", InstructionError::IncompatibleClassChangeError);
        }

        let field = field.borrow();
        let descriptor = field.descriptor();
        let object_ref_id = field.object_ref_id() as usize;
        let object_refs = class.borrow_mut().static_vars();
        let stack = frame.operand_stack_mut();

        if descriptor.starts_with('Z')
            || descriptor.starts_with('B')
            || descriptor.starts_with('C')
            || descriptor.starts_with('S')
            || descriptor.starts_with('I')
        {
            stack.push_int(object_refs.borrow().get_int(object_ref_id));
        } else if descriptor.starts_with('F') {
            stack.push_float(object_refs.borrow().get_float(object_ref_id));
        } else if descriptor.starts_with('J') {
            stack.push_long(object_refs.borrow().get_long(object_ref_id));
        } else if descriptor.starts_with('D') {
            stack.push_double(object_refs.borrow().get_double(object_ref_id));
        } else if descriptor.starts_with('L') || descriptor.starts_with('[') {
            stack.push_ref(object_refs.borrow().get_ref(object_ref_id));
        } else {
            // TODO
        }
    }
}
