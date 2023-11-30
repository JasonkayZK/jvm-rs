//! ASTORE instruction implementation
//!
//! ASTORE instruction stores object references to local variable
//!
//! List:
//!  - ASTORE
//!  - ASTORE_0
//!  - ASTORE_1
//!  - ASTORE_2
//!  - ASTORE_3
//!
#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Store reference into local variable
#[derive(Default, Debug)]
pub struct ASTORE {
    pub index: usize,
}

impl ASTORE {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl Instruction for ASTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        _astore(frame, self.index);
    }
}

#[derive(Default, Debug)]
pub struct ASTORE_0;

impl Instruction for ASTORE_0 {
    fn execute(&mut self, frame: &mut Frame) {
        _astore(frame, 0);
    }
}

#[derive(Default, Debug)]
pub struct ASTORE_1;

impl Instruction for ASTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        _astore(frame, 1);
    }
}

#[derive(Default, Debug)]
pub struct ASTORE_2;

impl Instruction for ASTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        _astore(frame, 2);
    }
}

#[derive(Default, Debug)]
pub struct ASTORE_3;

impl Instruction for ASTORE_3 {
    fn execute(&mut self, frame: &mut Frame) {
        _astore(frame, 3);
    }
}

fn _astore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack_mut().pop_ref();
    frame.local_vars_mut().set_ref(index, val);
}
