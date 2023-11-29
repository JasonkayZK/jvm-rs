//! ConstantValue attribute definition
//!
//! ConstantValue_attribute {
//!     u2 attribute_name_index;
//!     u4 attribute_length;
//!     u2 constant_value_index;
//! }

use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::classfile::constant_pool::ConstantPool;

use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct ConstantValueAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    constant_value_index: u16,
}

impl ConstantValueAttribute {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> ConstantValueAttribute {
        Self {
            constant_pool: cp,
            ..Default::default()
        }
    }
}

impl Display for ConstantValueAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ConstantValueAttribute]:\n\tconstant_value: {}",
            self.constant_pool
                .borrow()
                .get_utf8(self.constant_value_index)
        )
    }
}

impl AttributeInfo for ConstantValueAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.constant_value_index = reader.read_u16();
    }
}
