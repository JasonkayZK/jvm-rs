//! LOOKUP_SWITCH instruction implementation:
//!
//! Access jump table by key match and jump
//!
//! lookupswitch
//! <0-3 byte paddings>
//! defaultbyte1
//! defaultbyte2
//! defaultbyte3
//! defaultbyte4
//! npairs1
//! npairs2
//! npairs3
//! npairs4
//! match-offset pairs...
//!
#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::{branch, Instruction};
use crate::rtda::frame::Frame;

/// LOOKUP_SWITCH instruction
///
/// Note:
///  - npairs * 2 = len(match_offsets)
///
///  - match_offsets stores the indexes
///
///  - default_offset: the default branch's instruction offset in switch expression
///
#[derive(Default, Debug)]
pub struct LOOKUP_SWITCH {
    default_offset: i32,
    npairs: i32,
    match_offsets: Vec<i32>,
}

impl Instruction for LOOKUP_SWITCH {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        reader.skip_padding();
        self.default_offset = reader.read_i32();
        self.npairs = reader.read_i32();
        self.match_offsets = reader.read_i32s(self.npairs * 2);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let key = frame.operand_stack_mut().pop_int();
        let mut i = 0_i32;
        while i < self.npairs * 2 {
            if self.match_offsets[i as usize] == key {
                let offset = self.match_offsets[(i + 1) as usize];
                branch(frame, offset as i64);
                return;
            }
            i += 2;
        }
        branch(frame, self.default_offset as i64);
    }
}
