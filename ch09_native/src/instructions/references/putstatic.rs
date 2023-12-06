#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::{init_class, Instruction};
use crate::instructions::errors::InstructionError;
use crate::rtda::frame::Frame;
use crate::rtda::heap::field_ref::FieldRef;

/// Set static field in class
#[derive(Default, Debug)]
pub struct PUT_STATIC {
    index: u64,
}

impl Instruction for PUT_STATIC {
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
            .resolved_field(current_class.clone());

        let class = field.borrow().get_class().unwrap();

        if !class.borrow().init_started() {
            frame.revert_next_pc();
            init_class(&frame.thread(), &class);
            return;
        }

        if !field.borrow().is_static() {
            panic!("{}", InstructionError::IncompatibleClassChangeError);
        }
        if field.borrow().is_final()
            && (current_class.ne(&class) || current_method.borrow().name() != "<clinit>")
        {
            panic!("{}", InstructionError::IllegalAccessError);
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
            object_refs
                .borrow_mut()
                .set_int(object_ref_id, stack.pop_int());
        } else if descriptor.starts_with('F') {
            object_refs
                .borrow_mut()
                .set_float(object_ref_id, stack.pop_float());
        } else if descriptor.starts_with('J') {
            object_refs
                .borrow_mut()
                .set_long(object_ref_id, stack.pop_long());
        } else if descriptor.starts_with('D') {
            object_refs
                .borrow_mut()
                .set_double(object_ref_id, stack.pop_double());
        } else if descriptor.starts_with('L') || descriptor.starts_with('[') {
            object_refs
                .borrow_mut()
                .set_ref(object_ref_id, stack.pop_ref());
        } else {
            // TODO
        }
    }
}
