//! LineNumberTable attribute definition
//!
//! LineNumberTable_attribute {
//!     u2 attribute_name_index;
//!     u4 attribute_length;
//!     u2 line_number_table_length;
//!     {   u2 start_pc;
//!         u2 line_number;
//!     } line_number_table[line_number_table_length];
//! }
use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::classfile::attribute_info::types::AttributeTypeNameEnum;
use crate::classfile::constant_pool::ConstantPool;

use super::{AttributeInfo, ClassReader};

#[derive(Clone)]
pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

impl Display for LineNumberTableEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[LineNumberTableEntry], start_pc: {}, line_number: {}",
            self.start_pc, self.line_number
        )
    }
}

#[derive(Default, Clone)]
pub struct LineNumberTableAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    line_number_table: Vec<LineNumberTableEntry>,
}

impl LineNumberTableAttribute {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> LineNumberTableAttribute {
        Self {
            constant_pool: cp,
            ..Default::default()
        }
    }
}

impl Display for LineNumberTableAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[LineNumberTableAttribute]:\n\t{}",
            self.line_number_table
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("\n\t\t")
        )
    }
}

impl LineNumberTableAttribute {
    pub fn get_line_number(&self, pc: i64) -> i64 {
        for i in (0..self.line_number_table.len()).rev() {
            let entry = self.line_number_table.get(i);
            if pc >= entry.as_ref().unwrap().start_pc as i64 {
                return entry.unwrap().line_number as i64;
            }
        }

        -1
    }
}

impl AttributeInfo for LineNumberTableAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let line_number_table_length = reader.read_u16();
        let mut line_number_table = vec![];
        for _ in 0..line_number_table_length {
            line_number_table.push(LineNumberTableEntry {
                start_pc: reader.read_u16(),
                line_number: reader.read_u16(),
            });
        }
        self.line_number_table = line_number_table;
    }

    fn name(&self) -> &str {
        AttributeTypeNameEnum::LineNumberTable.into()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
