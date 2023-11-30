#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::{branch, Instruction};
use crate::rtda::frame::Frame;

/// Branch if reference is null
#[derive(Default, Debug)]
pub struct IFNULL {
    pub offset: i64,
}

impl Instruction for IFNULL {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        if frame.operand_stack_mut().pop_ref() == None {
            branch(frame, self.offset);
        }
    }
}

/// Branch if reference not null
#[derive(Default, Debug)]
pub struct IFNONNULL {
    pub offset: i64,
}

impl Instruction for IFNONNULL {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        if frame.operand_stack_mut().pop_ref() != None {
            branch(frame, self.offset);
        }
    }
}
