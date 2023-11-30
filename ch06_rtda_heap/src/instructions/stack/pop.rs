//! POP instructions implementation
//!
//! List:
//!  - POP: Pop the top operand stack value
//!  - POP2: Pop the top one or two operand stack values
//!
#![allow(non_camel_case_types)]

use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Pop the top operand stack value
#[derive(Default, Debug)]
pub struct POP;

impl Instruction for POP {
    /// bottom -> top
    /// [...][c][b][a]
    ///             |
    ///             V
    /// [...][c][b]
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().pop_ref();
    }
}

/// Pop the top one or two operand stack values
#[derive(Default, Debug)]
pub struct POP2;

impl Instruction for POP2 {
    /// bottom -> top
    /// [...][c][b][a]
    ///          |  |
    ///          V  V
    /// [...][c]
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        stack.pop_ref();
        stack.pop_ref();
    }
}
