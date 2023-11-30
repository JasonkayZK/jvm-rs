//! Exceptions attribute definition
//!
//! Exceptions_attribute {
//!     u2 attribute_name_index;
//!     u4 attribute_length;
//!     u2 number_of_exceptions;
//!     u2 exception_index_table[number_of_exceptions];
//! }

use crate::classfile::attribute_info::types::AttributeTypeNameEnum;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::classfile::constant_pool::ConstantPool;

use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct ExceptionsAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    exception_index_table: Vec<u16>,
}

impl ExceptionsAttribute {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> ExceptionsAttribute {
        Self {
            constant_pool: cp,
            ..Default::default()
        }
    }
}

impl Display for ExceptionsAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut exceptions = Vec::with_capacity(self.exception_index_table.len());
        for exception_index in &self.exception_index_table {
            exceptions.push(self.constant_pool.borrow().get_utf8(*exception_index))
        }

        write!(
            f,
            "[ExceptionsAttribute]:\n\t, {}",
            exceptions.join("\n\t\t")
        )
    }
}

impl AttributeInfo for ExceptionsAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.exception_index_table = reader.read_u16s();
    }

    fn name(&self) -> &str {
        AttributeTypeNameEnum::Exceptions.into()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
