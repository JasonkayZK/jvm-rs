//! Push item from runtime constant pool to operand stack.
//!

#![allow(non_camel_case_types, clippy::upper_case_acronyms)]

use crate::classfile::constant_pool::consts;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;
use crate::rtda::heap::string_pool::StringPool;

/// Push item from run-time constant pool
#[derive(Default, Debug)]
pub struct LDC {
    index: u64,
}

impl Instruction for LDC {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        _ldc(frame, self.index);
    }
}

/// Push item from run-time constant pool (wide index)
#[derive(Default, Debug)]
pub struct LDC_W {
    index: u64,
}

impl Instruction for LDC_W {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        _ldc(frame, self.index);
    }
}

/// Push long or double from run-time constant pool (wide index)
#[derive(Default, Debug)]
pub struct LDC2_W {
    index: u64,
}

impl Instruction for LDC2_W {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let index = self.index;
        let method = frame.method();
        let stack = frame.operand_stack_mut();
        let current_class = method.borrow().get_class();
        let r_cp = current_class.borrow_mut().constant_pool();
        let tag = r_cp.borrow().get_constant(index as usize).tag();

        match tag {
            consts::CONSTANT_LONG => {
                let val = *r_cp
                    .borrow()
                    .get_constant(index as usize)
                    .as_any()
                    .downcast_ref::<i64>()
                    .unwrap();
                stack.push_long(val);
            }
            consts::CONSTANT_DOUBLE => {
                let val = *r_cp
                    .borrow()
                    .get_constant(index as usize)
                    .as_any()
                    .downcast_ref::<f64>()
                    .unwrap();
                stack.push_double(val);
            }
            _ => {
                panic!("java.lang.ClassFormatError");
            }
        }
    }
}

fn _ldc(frame: &mut Frame, index: u64) {
    let method = frame.method();
    let stack = frame.operand_stack_mut();
    let current_class = method.borrow().get_class();
    let r_cp = current_class.borrow_mut().constant_pool();
    let tag = r_cp.borrow().get_constant(index as usize).tag();

    match tag {
        consts::CONSTANT_INTEGER => {
            let val = *r_cp
                .borrow()
                .get_constant(index as usize)
                .as_any()
                .downcast_ref::<i32>()
                .unwrap();
            stack.push_int(val);
        }
        consts::CONSTANT_FLOAT => {
            let val = *r_cp
                .borrow()
                .get_constant(index as usize)
                .as_any()
                .downcast_ref::<f32>()
                .unwrap();
            stack.push_float(val);
        }
        consts::CONSTANT_STRING => {
            let val = &*r_cp
                .borrow_mut()
                .get_constant(index as usize)
                .as_any()
                .downcast_ref::<String>()
                .unwrap()
                .clone();
            let interned_str = StringPool::global()
                .lock()
                .unwrap()
                .jstring(current_class.borrow().loader().unwrap(), val.into());
            stack.push_ref(Some(interned_str));
        }
        _ => {
            panic!("TODO: ldc!");
        }
    }
}
