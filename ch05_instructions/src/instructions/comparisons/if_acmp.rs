#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::{branch, Instruction};
use crate::rtda::frame::Frame;

/// Branch if reference comparison succeeds
#[derive(Default, Debug)]
pub struct IF_ACMPEQ {
    pub offset: i64,
}

impl Instruction for IF_ACMPEQ {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        if _acmp(frame) {
            branch(frame, self.offset);
        }
    }
}

#[derive(Default, Debug)]
pub struct IF_ACMPNE {
    pub offset: i64,
}

impl Instruction for IF_ACMPNE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        if !_acmp(frame) {
            branch(frame, self.offset);
        }
    }
}

fn _acmp(frame: &mut Frame) -> bool {
    let stack = frame.operand_stack_mut();
    let ref2 = stack.pop_ref();
    let ref1 = stack.pop_ref();
    ref1.eq(&ref2)
}
