//! CONSTANT_Class definition
//!
//! CONSTANT_Class_info {
//!     u1 tag;
//!     u2 name_index;
//! }

use std::cell::RefCell;
use std::rc::Rc;

use crate::classfile::constant_pool::consts::CONSTANT_CLASS;

use super::{ClassReader, ConstantInfo, ConstantPool};

#[derive(Clone)]
pub struct ConstantClassInfo {
    constant_pool: Rc<RefCell<ConstantPool>>,
    name_index: u16,
}

impl ConstantInfo for ConstantClassInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.name_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        CONSTANT_CLASS
    }
}

impl ConstantClassInfo {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        ConstantClassInfo {
            constant_pool: cp,
            name_index: 0,
        }
    }

    pub fn name(&self) -> String {
        self.constant_pool.borrow().get_utf8(self.name_index)
    }
}
