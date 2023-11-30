#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Load long from local variable
#[derive(Default, Debug)]
pub struct LLOAD {
    pub index: usize,
}

impl LLOAD {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl Instruction for LLOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        _lload(frame, self.index);
    }
}

#[derive(Default, Debug)]
pub struct LLOAD_0;

impl Instruction for LLOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        _lload(frame, 0);
    }
}

#[derive(Default, Debug)]
pub struct LLOAD_1;

impl Instruction for LLOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        _lload(frame, 1);
    }
}

#[derive(Default, Debug)]
pub struct LLOAD_2;

impl Instruction for LLOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        _lload(frame, 2);
    }
}

#[derive(Default, Debug)]
pub struct LLOAD_3;

impl Instruction for LLOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        _lload(frame, 3);
    }
}

fn _lload(frame: &mut Frame, index: usize) {
    let val = frame.local_vars_mut().get_long(index);
    frame.operand_stack_mut().push_long(val);
}
