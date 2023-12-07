#![allow(non_camel_case_types)]

use std::cmp::Ordering;

use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Compare long
#[derive(Default, Debug)]
pub struct LCMP;

impl Instruction for LCMP {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        match v1.cmp(&v2) {
            Ordering::Less => stack.push_int(-1),
            Ordering::Equal => stack.push_int(0),
            Ordering::Greater => stack.push_int(1),
        }
    }
}
