#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Increment local variable by constant
#[derive(Default, Debug)]
pub struct IINC {
    pub index: usize,
    pub const_val: i32,
}

impl IINC {
    pub fn new(index: usize, const_val: i32) -> Self {
        Self { index, const_val }
    }
}

impl Instruction for IINC {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
        self.const_val = reader.read_i8() as i32;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let local_vars = frame.local_vars_mut();
        let val = local_vars.get_int(self.index);
        let val = val + self.const_val;
        local_vars.set_int(self.index, val);
    }
}
