/// InnerClass_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 number_of_classes;
///     {   u2 inner_class_info_index;
///         u2 outer_class_info_index;
///         u2 inner_name_index;
///         u2 inner_class_access_flags;
///     } classes[number_of_classes];
/// }
///
///
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::classfile::constant_pool::ConstantPool;

use super::{AttributeInfo, ClassReader};

pub struct InnerClassInfo {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16,
}

#[derive(Default)]
pub struct InnerClassesAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    classes: Vec<InnerClassInfo>,
}

impl InnerClassesAttribute {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> InnerClassesAttribute {
        Self {
            constant_pool: cp,
            ..Default::default()
        }
    }
}

impl Display for InnerClassesAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[InnerClassesAttribute]")
    }
}

impl AttributeInfo for InnerClassesAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let number_of_classes = reader.read_u16();
        let mut classes = vec![];
        for _i in 0..number_of_classes {
            classes.push(InnerClassInfo {
                inner_class_info_index: reader.read_u16(),
                outer_class_info_index: reader.read_u16(),
                inner_name_index: reader.read_u16(),
                inner_class_access_flags: reader.read_u16(),
            });
        }
        self.classes = classes;
    }
}
