#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::instructions::errors::InstructionError;
use crate::rtda::frame::Frame;
use crate::rtda::heap::field_ref::FieldRef;

/// Get field from object
#[derive(Default, Debug)]
pub struct GET_FIELD {
    index: u64,
}

impl Instruction for GET_FIELD {
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

        if field.borrow().is_static() {
            panic!("{}", InstructionError::IncompatibleClassChangeError);
        }

        let stack = frame.operand_stack_mut();
        let _ref = stack.pop_ref();
        if _ref.is_none() {
            panic!("{}", InstructionError::NullPointerException);
        }

        let field = field.borrow();
        let descriptor = field.descriptor();
        let object_ref_id = field.object_ref_id() as usize;

        if descriptor.starts_with('Z')
            || descriptor.starts_with('B')
            || descriptor.starts_with('C')
            || descriptor.starts_with('S')
            || descriptor.starts_with('I')
        {
            stack.push_int(_ref.unwrap().borrow().fields().get_int(object_ref_id));
        } else if descriptor.starts_with('F') {
            stack.push_float(_ref.unwrap().borrow().fields().get_float(object_ref_id));
        } else if descriptor.starts_with('J') {
            stack.push_long(_ref.unwrap().borrow().fields().get_long(object_ref_id));
        } else if descriptor.starts_with('D') {
            stack.push_double(_ref.unwrap().borrow().fields().get_double(object_ref_id));
        } else if descriptor.starts_with('L') || descriptor.starts_with('[') {
            stack.push_ref(_ref.unwrap().borrow().fields().get_ref(object_ref_id));
        } else {
            // TODO
        }
    }
}
