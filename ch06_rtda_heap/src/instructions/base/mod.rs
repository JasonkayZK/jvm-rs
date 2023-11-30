use std::fmt::Debug;

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::rtda::frame::Frame;

pub mod bytecode_reader;

pub trait Instruction: Debug {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        // Default operation does nothing
    }

    /// Execute the current frame from stack
    fn execute(&mut self, frame: &mut Frame);
}

pub fn branch(frame: &mut Frame, offset: i64) {
    let pc = frame.thread().borrow().pc();
    let next_pc = pc + offset;
    frame.set_next_pc(next_pc);
}
