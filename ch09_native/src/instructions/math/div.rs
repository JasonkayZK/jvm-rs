#![allow(non_camel_case_types)]

use crate::instructions::base::Instruction;
use crate::instructions::errors::InstructionError;
use crate::rtda::frame::Frame;

/// Divide double
#[derive(Default, Debug)]
pub struct DDIV;

impl Instruction for DDIV {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        let result = v1 / v2;
        stack.push_double(result);
    }
}

/// Divide float
#[derive(Default, Debug)]
pub struct FDIV;

impl Instruction for FDIV {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        let result = v1 / v2;
        stack.push_float(result);
    }
}

/// Divide int
#[derive(Default, Debug)]
pub struct IDIV;

impl Instruction for IDIV {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        if v2 == 0 {
            panic!("{}", InstructionError::ArithmeticDividedByZero)
        }
        let result = v1 / v2;
        stack.push_int(result);
    }
}

/// Divide long
#[derive(Default, Debug)]
pub struct LDIV;

impl Instruction for LDIV {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        if v2 == 0 {
            panic!("{}", InstructionError::ArithmeticDividedByZero)
        }
        let result = v1 / v2;
        stack.push_long(result);
    }
}
