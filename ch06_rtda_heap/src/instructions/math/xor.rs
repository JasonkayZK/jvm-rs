#![allow(non_camel_case_types)]

use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Boolean XOR int
#[derive(Default, Debug)]
pub struct IXOR;

impl Instruction for IXOR {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let v1 = stack.pop_int();
        let v2 = stack.pop_int();
        let result = v1 ^ v2;
        stack.push_int(result);
    }
}

/// Boolean XOR long
#[derive(Default, Debug)]
pub struct LXOR;

impl Instruction for LXOR {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let v1 = stack.pop_long();
        let v2 = stack.pop_long();
        let result = v1 ^ v2;
        stack.push_long(result);
    }
}
