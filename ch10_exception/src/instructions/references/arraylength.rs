#![allow(non_camel_case_types)]

use crate::instructions::base::Instruction;
use crate::instructions::errors::InstructionError;
use crate::rtda::frame::Frame;

/// Get length of array
#[derive(Default, Debug)]
pub struct ARRAY_LENGTH;

impl Instruction for ARRAY_LENGTH {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let _ref = stack.pop_ref();
        if _ref.is_none() {
            panic!("{}", InstructionError::NullPointerException);
        }

        let arr_len = _ref.unwrap().borrow().array_length();
        stack.push_int(arr_len as i32);
    }
}
