#![allow(non_camel_case_types)]

use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Convert long to double
#[derive(Default, Debug)]
pub struct L2D;

impl Instruction for L2D {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let l = stack.pop_long();
        let d = l as f64;
        stack.push_double(d);
    }
}

/// Convert long to float
#[derive(Default, Debug)]
pub struct L2F;

impl Instruction for L2F {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let l = stack.pop_long();
        let f = l as f32;
        stack.push_float(f);
    }
}

/// Convert long to int
#[derive(Default, Debug)]
pub struct L2I;

impl Instruction for L2I {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let l = stack.pop_long();
        let i = l as i32;
        stack.push_int(i);
    }
}
