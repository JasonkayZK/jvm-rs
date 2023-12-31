#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Store int into local variable
#[derive(Default, Debug)]
pub struct ISTORE {
    pub index: usize,
}

impl ISTORE {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl Instruction for ISTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        _istore(frame, self.index);
    }
}

#[derive(Default, Debug)]
pub struct ISTORE_0;

impl Instruction for ISTORE_0 {
    fn execute(&mut self, frame: &mut Frame) {
        _istore(frame, 0);
    }
}

#[derive(Default, Debug)]
pub struct ISTORE_1;

impl Instruction for ISTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        _istore(frame, 1);
    }
}

#[derive(Default, Debug)]
pub struct ISTORE_2;

impl Instruction for ISTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        _istore(frame, 2);
    }
}

#[derive(Default, Debug)]
pub struct ISTORE_3;

impl Instruction for ISTORE_3 {
    fn execute(&mut self, frame: &mut Frame) {
        _istore(frame, 3);
    }
}

fn _istore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack_mut().pop_int();
    frame.local_vars_mut().set_int(index, val);
}
