#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::{branch, Instruction};
use crate::rtda::frame::Frame;

/// Goto execute specific offset instruction with no condition
#[derive(Default, Debug)]
pub struct GOTO {
    pub offset: i64,
}

impl Instruction for GOTO {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        branch(frame, self.offset);
    }
}
