//! SourceFile attribute definition
//!
//! SourceFile_attribute {
//!     u2 attribute_name_index;
//!     u4 attribute_length;
//!     u2 source_file_index;
//! }

use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct SourceFileAttribute {
    source_file_index: u16,
}

impl AttributeInfo for SourceFileAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.source_file_index = reader.read_u16();
    }
}

impl SourceFileAttribute {
    pub fn new() -> Self {
        let mut sfa = SourceFileAttribute::default();
        sfa
    }
}
