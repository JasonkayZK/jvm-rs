//! NOP instruction implementation
//!
//! NOP does nothing
use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

#[derive(Default, Debug)]
pub struct NOP;

impl Instruction for NOP {
    fn execute(&mut self, _frame: &mut Frame) {
        // Really do nothing
    }
}
