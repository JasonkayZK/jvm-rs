//! SourceFile attribute definition
//!
//! SourceFile_attribute {
//!     u2 attribute_name_index;
//!     u4 attribute_length;
//!     u2 source_file_index;
//! }

use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::classfile::attribute_info::types::AttributeTypeNameEnum;
use crate::classfile::constant_pool::ConstantPool;

use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct SourceFileAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    source_file_index: u16,
}

impl Display for SourceFileAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[SourceFileAttribute]:\n\t{}",
            self.constant_pool.borrow().get_utf8(self.source_file_index)
        )
    }
}

impl AttributeInfo for SourceFileAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.source_file_index = reader.read_u16();
    }

    fn name(&self) -> &str {
        AttributeTypeNameEnum::SourceFile.into()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl SourceFileAttribute {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> SourceFileAttribute {
        Self {
            constant_pool: cp,
            ..Default::default()
        }
    }

    pub fn file_name(&self) -> String {
        self.constant_pool.borrow().get_utf8(self.source_file_index)
    }
}
