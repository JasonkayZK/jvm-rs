#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::{branch, Instruction};
use crate::rtda::frame::Frame;

/// Branch if int comparison with zero succeeds
#[derive(Default, Debug)]
pub struct IFEQ {
    pub offset: i64,
}

impl Instruction for IFEQ {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_int();
        if val == 0 {
            branch(frame, self.offset);
        }
    }
}

/// Branch if int comparison with non-zero succeeds
#[derive(Default, Debug)]
pub struct IFNE {
    pub offset: i64,
}

impl Instruction for IFNE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_int();
        if val != 0 {
            branch(frame, self.offset);
        }
    }
}

/// Less than
#[derive(Default, Debug)]
pub struct IFLT {
    pub offset: i64,
}

impl Instruction for IFLT {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_int();
        if val < 0 {
            branch(frame, self.offset);
        }
    }
}

/// Less equals
#[derive(Default, Debug)]
pub struct IFLE {
    pub offset: i64,
}

impl Instruction for IFLE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_int();
        if val <= 0 {
            branch(frame, self.offset);
        }
    }
}

/// Greater than
#[derive(Default, Debug)]
pub struct IFGT {
    pub offset: i64,
}

impl Instruction for IFGT {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_int();
        if val > 0 {
            branch(frame, self.offset);
        }
    }
}

/// Greater equals
#[derive(Default, Debug)]
pub struct IFGE {
    pub offset: i64,
}

impl Instruction for IFGE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack_mut().pop_int();
        if val >= 0 {
            branch(frame, self.offset);
        }
    }
}
