//! LocalVariableTable attribute definition
//!
//! LocalVariableTable_attribute {
//!     u2 attribute_name_index;
//!     u4 attribute_length;
//!     u2 local_variable_table_length;
//!     {   u2 start_pc;
//!         u2 length;
//!         u2 name_index;
//!         u2 descriptor_index;
//!         u2 index;
//!     } local_variable_table[local_variable_table_length];
//! }

use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::classfile::constant_pool::ConstantPool;

use super::{AttributeInfo, ClassReader};

pub struct LocalVariableTableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

#[derive(Default)]
pub struct LocalVariableTableAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    local_variable_table: Vec<LocalVariableTableEntry>,
}

impl LocalVariableTableAttribute {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> LocalVariableTableAttribute {
        Self {
            constant_pool: cp,
            ..Default::default()
        }
    }
}

impl Display for LocalVariableTableAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut entries = Vec::with_capacity(self.local_variable_table.len());
        let cp = self.constant_pool.borrow();
        for entry in &self.local_variable_table {
            entries.push(format!("[LocalVariableTableEntry]: start_pc: {}, length: {}, name: {}, descriptor: {}, index: {}",
                                 entry.start_pc,
                                 entry.length, cp.get_utf8(entry.name_index), cp.get_utf8(entry.descriptor_index), entry.index))
        }
        write!(
            f,
            "[LocalVariableTableAttribute]:\n\t{}",
            entries.join("\n\t\t")
        )
    }
}

impl AttributeInfo for LocalVariableTableAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let local_variable_table_length = reader.read_u16();
        let mut local_variable_table = vec![];
        for _ in 0..local_variable_table_length {
            local_variable_table.push(LocalVariableTableEntry {
                start_pc: reader.read_u16(),
                length: reader.read_u16(),
                name_index: reader.read_u16(),
                descriptor_index: reader.read_u16(),
                index: reader.read_u16(),
            });
        }
        self.local_variable_table = local_variable_table;
    }
}
