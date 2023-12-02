//! AttributeInfo definition in classfile
//!
//! attribute_info {
//!     u2 attribute_name_index;
//!     u4 attribute_length;
//!     u1 info[attribute_length];
//! }

use std::any::Any;
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
use crate::classfile::attribute_info::types::AttributeTypeNameEnum;
use crate::classfile::attribute_info::unparsed::UnparsedAttribute;
use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_pool::ConstantPool;

pub mod bootstrap_methods;
pub mod code;
pub mod constant_value;
pub mod enclosing_method;
pub mod exceptions;
pub mod inner_classes;
pub mod line_number;
pub mod local_variable_table;
pub mod local_variable_type_table;
pub mod markers;
pub mod signature;
pub mod source_file;
pub mod types;
pub mod unparsed;

pub trait AttributeInfo: Display {
    fn read_info(&mut self, reader: &mut ClassReader);

    /// Get attribute name
    fn name(&self) -> &str;

    /// For downcast use
    fn as_any(&self) -> &dyn Any;
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
    match attr_name.into() {
        AttributeTypeNameEnum::Code => Box::new(CodeAttribute::new(cp)),
        AttributeTypeNameEnum::ConstantValue => Box::new(ConstantValueAttribute::new(cp)),
        AttributeTypeNameEnum::Deprecated => Box::<DeprecatedAttribute>::default(),
        AttributeTypeNameEnum::Exceptions => Box::new(ExceptionsAttribute::new(cp)),
        AttributeTypeNameEnum::LineNumberTable => Box::new(LineNumberTableAttribute::new(cp)),
        AttributeTypeNameEnum::LocalVariableTable => Box::new(LocalVariableTableAttribute::new(cp)),
        AttributeTypeNameEnum::SourceFile => Box::new(SourceFileAttribute::new(cp)),
        AttributeTypeNameEnum::Synthetic => Box::<SyntheticAttribute>::default(),
        AttributeTypeNameEnum::Signature => Box::new(SignatureAttribute::new(cp)),
        AttributeTypeNameEnum::EnclosingMethod => Box::new(EnclosingMethodAttribute::new(cp)),
        AttributeTypeNameEnum::BootstrapMethods => Box::new(BootstrapMethodsAttribute::new(cp)),
        AttributeTypeNameEnum::InnerClasses => Box::new(InnerClassesAttribute::new(cp)),
        AttributeTypeNameEnum::LocalVariableTypeTable => {
            Box::new(LocalVariableTypeTableAttribute::new(cp))
        }
        _ => Box::new(UnparsedAttribute::new(
            attr_name.to_string(),
            attr_length,
            None,
        )),
    }
}
