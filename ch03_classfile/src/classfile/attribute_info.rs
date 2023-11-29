//! AttributeInfo definition in classfile
//!
//! attribute_info {
//!     u2 attribute_name_index;
//!     u4 attribute_length;
//!     u1 info[attribute_length];
//! }

mod bootstrap_method;
mod code;
mod constant_value;
mod enclosing_method;
mod exception;
mod inner_class;
mod line_number;
mod local_variable_table;
mod local_variable_type_table;
mod markers;
mod signature;
mod source_file;
mod unparsed;

use crate::classfile::attribute_info::bootstrap_method::BootstrapMethodAttribute;
use crate::classfile::attribute_info::code::CodeAttribute;
use crate::classfile::attribute_info::constant_value::ConstantValueAttribute;
use crate::classfile::attribute_info::enclosing_method::EnclosingMethodAttribute;
use crate::classfile::attribute_info::exception::ExceptionAttribute;
use crate::classfile::attribute_info::inner_class::InnerClassAttribute;
use crate::classfile::attribute_info::line_number::LineNumberTableAttribute;
use crate::classfile::attribute_info::local_variable_table::LocalVariableTableAttribute;
use crate::classfile::attribute_info::local_variable_type_table::LocalVariableTypeTableAttribute;
use crate::classfile::attribute_info::markers::{DeprecatedAttribute, SyntheticAttribute};
use crate::classfile::attribute_info::signature::SignatureAttribute;
use crate::classfile::attribute_info::source_file::SourceFileAttribute;
use crate::classfile::attribute_info::unparsed::UnparsedAttribute;
use std::cell::RefCell;
use std::rc::Rc;

use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_pool::ConstantPool;

pub trait AttributeInfo {
    fn read_info(&mut self, reader: &mut ClassReader);
}

pub fn read_attributes(
    reader: &mut ClassReader,
    cp: Rc<RefCell<ConstantPool>>,
) -> Vec<Box<dyn AttributeInfo>> {
    let attribute_count = reader.read_u16();
    let mut attributes = vec![];
    for _ in 0..attribute_count {
        attributes.push(read_attribute(reader, cp.clone()));
    }
    attributes
}

fn read_attribute(
    reader: &mut ClassReader,
    cp: Rc<RefCell<ConstantPool>>,
) -> Box<dyn AttributeInfo> {
    let attr_name_index = reader.read_u16();
    let attr_name = cp.borrow().get_utf8(attr_name_index);
    let attr_length = reader.read_u32();
    let mut attr_info = new_attribute(&attr_name, attr_length, cp);
    attr_info.read_info(reader);
    attr_info
}

fn new_attribute(
    attr_name: &str,
    attr_length: u32,
    cp: Rc<RefCell<ConstantPool>>,
) -> Box<dyn AttributeInfo> {
    match attr_name {
        "Code" => Box::new(CodeAttribute::new(cp)),
        "ConstantValue" => Box::<ConstantValueAttribute>::default(),
        "Deprecated" => Box::<DeprecatedAttribute>::default(),
        "Exceptions" => Box::<ExceptionAttribute>::default(),
        "LineNumberTable" => Box::<LineNumberTableAttribute>::default(),
        "LocalVariableTable" => Box::<LocalVariableTableAttribute>::default(),
        "SourceFile" => Box::new(SourceFileAttribute::new()),
        "Synthetic" => Box::<SyntheticAttribute>::default(),
        "Signature" => Box::<SignatureAttribute>::default(),
        "EnclosingMethod" => Box::<EnclosingMethodAttribute>::default(),
        "BootstrapMethod" => Box::<BootstrapMethodAttribute>::default(),
        "InnerClass" => Box::<InnerClassAttribute>::default(),
        "LocalVariableTypeTable" => Box::<LocalVariableTypeTableAttribute>::default(),
        _ => Box::new(UnparsedAttribute::new(
            attr_name.to_string(),
            attr_length,
            None,
        )),
    }
}
