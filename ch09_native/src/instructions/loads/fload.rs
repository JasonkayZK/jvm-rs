#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Load float from local variable
#[derive(Default, Debug)]
pub struct FLOAD {
    pub index: usize,
}

impl FLOAD {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl Instruction for FLOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        _fload(frame, self.index);
    }
}

#[derive(Default, Debug)]
pub struct FLOAD_0;

impl Instruction for FLOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        _fload(frame, 0);
    }
}

#[derive(Default, Debug)]
pub struct FLOAD_1;

impl Instruction for FLOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        _fload(frame, 1);
    }
}

#[derive(Default, Debug)]
pub struct FLOAD_2;

impl Instruction for FLOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        _fload(frame, 2);
    }
}

#[derive(Default, Debug)]
pub struct FLOAD_3;

impl Instruction for FLOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        _fload(frame, 3);
    }
}

fn _fload(frame: &mut Frame, index: usize) {
    let val = frame.local_vars_mut().get_float(index);
    frame.operand_stack_mut().push_float(val);
}
