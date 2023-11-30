//! CONSTANT_NameAndType definition
//!
//! CONSTANT_NameAndType_info {
//!     u1 tag;
//!     u2 name_index;
//!     u2 descriptor_index;
//! }

use crate::classfile::constant_pool::consts::CONSTANT_NAME_AND_TYPE;

use super::{ClassReader, ConstantInfo};

#[derive(Default)]
pub struct ConstantNameAndTypeInfo {
    name_index: u16,
    descriptor_index: u16,
}

impl ConstantInfo for ConstantNameAndTypeInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.name_index = reader.read_u16();
        self.descriptor_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        CONSTANT_NAME_AND_TYPE
    }
}
