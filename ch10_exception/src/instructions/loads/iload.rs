#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Load int from local variable
#[derive(Default, Debug)]
pub struct ILOAD {
    pub index: usize,
}

impl ILOAD {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl Instruction for ILOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        _iload(frame, self.index);
    }
}

#[derive(Default, Debug)]
pub struct ILOAD_0;

impl Instruction for ILOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        _iload(frame, 0);
    }
}

#[derive(Default, Debug)]
pub struct ILOAD_1;

impl Instruction for ILOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        _iload(frame, 1);
    }
}

#[derive(Default, Debug)]
pub struct ILOAD_2;

impl Instruction for ILOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        _iload(frame, 2);
    }
}

#[derive(Default, Debug)]
pub struct ILOAD_3;

impl Instruction for ILOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        _iload(frame, 3);
    }
}

fn _iload(frame: &mut Frame, index: usize) {
    let val = frame.local_vars_mut().get_int(index);
    frame.operand_stack_mut().push_int(val);
}
