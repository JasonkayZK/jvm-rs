#![allow(non_camel_case_types)]

use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Subtract double
#[derive(Default, Debug)]
pub struct DSUB;

impl Instruction for DSUB {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        let result = v1 - v2;
        stack.push_double(result);
    }
}

/// Subtract float
#[derive(Default, Debug)]
pub struct FSUB;

impl Instruction for FSUB {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        let result = v1 - v2;
        stack.push_float(result);
    }
}

/// Subtract int
#[derive(Default, Debug)]
pub struct ISUB;

impl Instruction for ISUB {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let result = v1.wrapping_sub(v2);
        stack.push_int(result);
    }
}

/// Subtract long
#[derive(Default, Debug)]
pub struct LSUB;

impl Instruction for LSUB {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        let result = v1.wrapping_sub(v2);
        stack.push_long(result);
    }
}
