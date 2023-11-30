#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Load reference from local variable
#[derive(Default, Debug)]
pub struct ALOAD {
    pub index: usize,
}

impl Instruction for ALOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        _aload(frame, self.index);
    }
}

#[derive(Default, Debug)]
pub struct ALOAD_0;

impl Instruction for ALOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        _aload(frame, 0);
    }
}

#[derive(Default, Debug)]
pub struct ALOAD_1;

impl Instruction for ALOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        _aload(frame, 1);
    }
}

#[derive(Default, Debug)]
pub struct ALOAD_2;

impl Instruction for ALOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        _aload(frame, 2);
    }
}

#[derive(Default, Debug)]
pub struct ALOAD_3;

impl Instruction for ALOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        _aload(frame, 3);
    }
}

fn _aload(frame: &mut Frame, index: usize) {
    let val = frame.local_vars_mut().get_ref(index);
    frame.operand_stack_mut().push_ref(val);
}
