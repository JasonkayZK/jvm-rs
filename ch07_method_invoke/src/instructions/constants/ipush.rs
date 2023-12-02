//! IPUSH instruction implementation
//!
//! List:
//!  - BIPUSH: Push byte
//!  - SIPUSH: Push short
//!

#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Push byte
#[derive(Default, Debug)]
pub struct BIPUSH {
    val: i8,
}

impl Instruction for BIPUSH {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.val = reader.read_i8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_int(self.val as i32);
    }
}

/// Push short
#[derive(Default, Debug)]
pub struct SIPUSH {
    val: i16,
}

impl Instruction for SIPUSH {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.val = reader.read_i16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_int(self.val as i32);
    }
}
