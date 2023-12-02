//! Code attribute definition
//!
//! Code_attribute {
//!     u2 attribute_name_index;
//!     u4 attribute_length;
//!     u2 max_stack;
//!     u2 max_locals;
//!     u4 code_length;
//!     u1 code[code_length];
//!     u2 exception_table_length;
//!     {   u2 start_pc;
//!         u2 end_pc;
//!         u2 handler_pc;
//!         u2 catch_type;
//!     } exception_table[exception_table_length];
//!     u2 attributes_count;
//!     attribute_info attributes[attributes_count];
//! }

use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::classfile::attribute_info::types::AttributeTypeNameEnum;

use super::ConstantPool;
use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct CodeAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<ExceptionTableEntry>,
    attributes: Vec<Box<dyn AttributeInfo>>,
}

impl Display for CodeAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CodeAttribute]: \n\tmax_stack: {}, \n\tmax_locals: {}, \n\tcode_len: {}, \n\texception_table: \n\t\t{}, \n\tattributes: \n\t\t{}",
               self.max_stack, self.max_locals, self.code.len(), self.exception_table.iter().map(|n| n.to_string()).collect::<Vec<String>>().join("\n\t\t"), self.attributes.iter().map(|n| n.to_string()).collect::<Vec<String>>().join("\n\t\t"))
    }
}

impl AttributeInfo for CodeAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.max_stack = reader.read_u16();
        self.max_locals = reader.read_u16();
        let code_length = reader.read_u32() as usize;
        self.code = reader.read_bytes(code_length);
        self.exception_table = read_exception_table(reader);
        self.attributes = super::read_attributes(reader, self.constant_pool.clone())
    }

    fn name(&self) -> &str {
        AttributeTypeNameEnum::Code.into()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CodeAttribute {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        Self {
            constant_pool: cp,
            ..Default::default()
        }
    }

    pub fn max_locals(&self) -> u16 {
        self.max_locals
    }

    pub fn max_stack(&self) -> u16 {
        self.max_stack
    }

    pub fn code(&self) -> Vec<u8> {
        self.code.clone()
    }
}

pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl Display for ExceptionTableEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[CodeAttribute]: \n\tstart_pc: {}, \n\tend_pc: {}, \n\thandler_pc: {}, \n\tcatch_type: {}",
            self.start_pc, self.end_pc, self.handler_pc, self.catch_type
        )
    }
}

fn read_exception_table(reader: &mut ClassReader) -> Vec<ExceptionTableEntry> {
    let exception_length = reader.read_u16();
    let mut exception_table = vec![];
    for _i in 0..exception_length {
        exception_table.push(ExceptionTableEntry {
            start_pc: reader.read_u16(),
            end_pc: reader.read_u16(),
            handler_pc: reader.read_u16(),
            catch_type: reader.read_u16(),
        });
    }
    exception_table
}
