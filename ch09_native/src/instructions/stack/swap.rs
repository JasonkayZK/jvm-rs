//! SWAP instruction implementation
//!
//! SWAP instruction swaps the top two operand stack values:
//!
//! bottom -> top
//! [...][c][b][a]
//!           \/
//!           /\
//!          V  V
//! [...][c][a][b]
//!
#![allow(non_camel_case_types)]

use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Swap the top two operand stack values
#[derive(Default, Debug)]
pub struct SWAP;

impl Instruction for SWAP {
    /// bottom -> top
    /// [...][c][b][a]
    ///           \/
    ///           /\
    ///          V  V
    /// [...][c][a][b]
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let reference1 = stack.pop_var();
        let reference2 = stack.pop_var();
        stack.push_var(reference1);
        stack.push_var(reference2);
    }
}
