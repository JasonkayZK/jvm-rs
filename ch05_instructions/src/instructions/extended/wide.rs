#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

use super::super::loads::*;
use super::super::math::*;
use super::super::stores::*;

/// Extend local variable index by additional bytes
#[derive(Default, Debug)]
pub struct WIDE {
    modified_instruction: Option<Box<dyn Instruction>>,
}

impl Instruction for WIDE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        let opcode = reader.read_u8();
        self.modified_instruction = match opcode {
            0x15 => Some(Box::new(ILOAD::new(reader.read_u16() as usize))),
            0x16 => Some(Box::new(LLOAD::new(reader.read_u16() as usize))),
            0x17 => Some(Box::new(FLOAD::new(reader.read_u16() as usize))),
            0x18 => Some(Box::new(DLOAD::new(reader.read_u16() as usize))),
            0x19 => Some(Box::new(ALOAD::new(reader.read_u16() as usize))),
            0x36 => Some(Box::new(ISTORE::new(reader.read_u16() as usize))),
            0x37 => Some(Box::new(LSTORE::new(reader.read_u16() as usize))),
            0x38 => Some(Box::new(FSTORE::new(reader.read_u16() as usize))),
            0x39 => Some(Box::new(DSTORE::new(reader.read_u16() as usize))),
            0x3a => Some(Box::new(ASTORE::new(reader.read_u16() as usize))),
            0x84 => {
                let index = reader.read_u16() as usize;
                let const_val = reader.read_i16() as i32;
                Some(Box::new(IINC::new(index, const_val)))
            }
            // Ret
            0xa9 => {
                panic!("{}", "Unsupported opcode: 0xa9!");
            }
            _ => {
                panic!("Unsupported opcode: 0x{:x}!", opcode);
            }
        }
    }

    fn execute(&mut self, frame: &mut Frame) {
        self.modified_instruction.as_mut().unwrap().execute(frame);
    }
}
