#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::instructions::errors::InstructionError;
use crate::rtda::frame::Frame;
use crate::rtda::heap::field_ref::FieldRef;

/// Set field in object
#[derive(Default, Debug)]
pub struct PUT_FIELD {
    index: u64,
}

impl Instruction for PUT_FIELD {
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

        if field.borrow().is_static() {
            panic!("{}", InstructionError::IncompatibleClassChangeError);
        }
        if field.borrow().is_final()
            && (current_class.ne(&class) || current_method.borrow().name() != "<init>")
        {
            panic!("{}", InstructionError::IllegalAccessError);
        }

        let field = field.borrow();
        let descriptor = field.descriptor();
        let object_ref_id = field.object_ref_id() as usize;
        let stack = frame.operand_stack_mut();

        if descriptor.starts_with('Z')
            || descriptor.starts_with('B')
            || descriptor.starts_with('C')
            || descriptor.starts_with('S')
            || descriptor.starts_with('I')
        {
            let val = stack.pop_int();
            let _ref = stack.pop_ref();
            if _ref.is_none() {
                panic!("{}", InstructionError::NullPointerException);
            }
            let obj = _ref.unwrap();
            obj.borrow_mut().fields_mut().set_int(object_ref_id, val);
        } else if descriptor.starts_with('F') {
            let val = stack.pop_float();
            let _ref = stack.pop_ref();
            if _ref.is_none() {
                panic!("{}", InstructionError::NullPointerException);
            }
            let mut obj = _ref.unwrap();
            obj.borrow_mut().fields_mut().set_float(object_ref_id, val);
        } else if descriptor.starts_with('J') {
            let val = stack.pop_long();
            let _ref = stack.pop_ref();
            if _ref.is_none() {
                panic!("{}", InstructionError::NullPointerException);
            }
            let mut obj = _ref.unwrap();
            obj.borrow_mut().fields_mut().set_long(object_ref_id, val);
        } else if descriptor.starts_with('D') {
            let val = stack.pop_double();
            let _ref = stack.pop_ref();
            if _ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let mut obj = _ref.unwrap();
            obj.borrow_mut().fields_mut().set_double(object_ref_id, val);
        } else if descriptor.starts_with('L') || descriptor.starts_with('[') {
            let val = stack.pop_ref();
            let _ref = stack.pop_ref();
            if _ref.is_none() {
                panic!("java.lang.NullPointerException");
            }
            let mut obj = _ref.unwrap();
            obj.borrow_mut().fields_mut().set_ref(object_ref_id, val);
        } else {
            // TODO
        }
    }
}
