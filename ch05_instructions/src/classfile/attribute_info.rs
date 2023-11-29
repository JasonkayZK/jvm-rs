//! AttributeInfo definition in classfile
//!
//! attribute_info {
//!     u2 attribute_name_index;
//!     u4 attribute_length;
//!     u1 info[attribute_length];
//! }

use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::classfile::attribute_info::bootstrap_methods::BootstrapMethodsAttribute;
use crate::classfile::attribute_info::code::CodeAttribute;
use crate::classfile::attribute_info::constant_value::ConstantValueAttribute;
use crate::classfile::attribute_info::enclosing_method::EnclosingMethodAttribute;
use crate::classfile::attribute_info::exceptions::ExceptionsAttribute;
use crate::classfile::attribute_info::inner_classes::InnerClassesAttribute;
use crate::classfile::attribute_info::line_number::LineNumberTableAttribute;
use crate::classfile::attribute_info::local_variable_table::LocalVariableTableAttribute;
use crate::classfile::attribute_info::local_variable_type_table::LocalVariableTypeTableAttribute;
use crate::classfile::attribute_info::markers::{DeprecatedAttribute, SyntheticAttribute};
use crate::classfile::attribute_info::signature::SignatureAttribute;
use crate::classfile::attribute_info::source_file::SourceFileAttribute;
use crate::classfile::attribute_info::unparsed::UnparsedAttribute;
use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_pool::ConstantPool;

mod bootstrap_methods;
mod code;
mod constant_value;
mod enclosing_method;
mod exceptions;
mod inner_classes;
mod line_number;
mod local_variable_table;
mod local_variable_type_table;
mod markers;
mod signature;
mod source_file;
mod unparsed;

pub trait AttributeInfo: Display {
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
        "ConstantValue" => Box::new(ConstantValueAttribute::new(cp)),
        "Deprecated" => Box::<DeprecatedAttribute>::default(),
        "Exceptions" => Box::new(ExceptionsAttribute::new(cp)),
        "LineNumberTable" => Box::new(LineNumberTableAttribute::new(cp)),
        "LocalVariableTable" => Box::new(LocalVariableTableAttribute::new(cp)),
        "SourceFile" => Box::new(SourceFileAttribute::new(cp)),
        "Synthetic" => Box::<SyntheticAttribute>::default(),
        "Signature" => Box::new(SignatureAttribute::new(cp)),
        "EnclosingMethod" => Box::new(EnclosingMethodAttribute::new(cp)),
        "BootstrapMethods" => Box::new(BootstrapMethodsAttribute::new(cp)),
        "InnerClasses" => Box::new(InnerClassesAttribute::new(cp)),
        "LocalVariableTypeTable" => Box::new(LocalVariableTypeTableAttribute::new(cp)),
        _ => Box::new(UnparsedAttribute::new(
            attr_name.to_string(),
            attr_length,
            None,
        )),
    }
}
