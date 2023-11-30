//! CONSTANT_String definition
//!
//! CONSTANT_String_info {
//!     u1 tag;
//!     u2 string_index;
//! }
use std::cell::RefCell;
use std::rc::Rc;

use crate::classfile::constant_pool::consts::CONSTANT_STRING;

use super::{ClassReader, ConstantInfo, ConstantPool};

pub struct ConstantStringInfo {
    constant_pool: Rc<RefCell<ConstantPool>>,
    string_index: u16,
}

impl ConstantInfo for ConstantStringInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.string_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        CONSTANT_STRING
    }
}

impl ConstantStringInfo {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        ConstantStringInfo {
            constant_pool: cp,
            string_index: 0,
        }
    }
}
