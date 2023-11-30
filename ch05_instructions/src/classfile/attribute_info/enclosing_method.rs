//! EnclosingMethod attribute definition
//!
//! EnclosingMethod_attribute {
//!     u2 attribute_name_index;
//!     u4 attribute_length;
//!     u2 class_index;
//!     u2 method_index;
//! }

use crate::classfile::attribute_info::types::AttributeTypeNameEnum;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use super::ConstantPool;
use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct EnclosingMethodAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    class_index: u16,
    method_index: u16,
}

impl Display for EnclosingMethodAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cp = self.constant_pool.borrow();
        write!(
            f,
            "[EnclosingMethodAttribute]:\n\tclass: {}\n\t\tmethod_index: {}",
            cp.get_utf8(self.class_index),
            cp.get_utf8(self.method_index)
        )
    }
}

impl AttributeInfo for EnclosingMethodAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.class_index = reader.read_u16();
        self.method_index = reader.read_u16();
    }

    fn name(&self) -> &str {
        AttributeTypeNameEnum::EnclosingMethod.into()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl EnclosingMethodAttribute {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        Self {
            constant_pool: cp,
            ..Default::default()
        }
    }
}
