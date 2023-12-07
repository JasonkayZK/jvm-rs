//! CONSTANT_Member_ref definitions
//!
//! Include:
//!
//! - CONSTANT_Field_Ref
//! - CONSTANT_Method_Ref
//! - CONSTANT_InterfaceMethod_Ref

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::{consts, ClassReader, ConstantInfo, ConstantPool};

/// Common Constant member reference definition for field, method and interface
pub struct ConstantMemberRefInfo {
    constant_pool: Rc<RefCell<ConstantPool>>,
    class_index: u16,
    name_and_type_index: u16,
}

impl ConstantMemberRefInfo {
    fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        ConstantMemberRefInfo {
            constant_pool: cp,
            class_index: 0,
            name_and_type_index: 0,
        }
    }

    fn read_info(&mut self, reader: &mut ClassReader) {
        self.class_index = reader.read_u16();
        self.name_and_type_index = reader.read_u16();
    }
}

/// CONSTANT_Field_Ref_info {
///     u1 tag;
///     u2 class_index;
///     u2 name_and_type_index;
/// }
pub struct ConstantFieldRefInfo {
    member_info: ConstantMemberRefInfo,
}

impl ConstantInfo for ConstantFieldRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.member_info.read_info(reader);
    }

    fn tag(&self) -> u8 {
        consts::CONSTANT_FIELD_REF
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ConstantFieldRefInfo {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        ConstantFieldRefInfo {
            member_info: ConstantMemberRefInfo::new(cp),
        }
    }

    pub fn class_name(&self) -> String {
        self.member_info
            .constant_pool
            .borrow()
            .get_class_name(self.member_info.class_index)
    }

    pub fn name_and_descriptor(&self) -> (String, String) {
        self.member_info
            .constant_pool
            .borrow()
            .get_name_and_type(self.member_info.name_and_type_index as usize)
    }
}

/// CONSTANT_Method_Ref_info {
///     u1 tag;
///     u2 class_index;
///     u2 name_and_type_index;
/// }
pub struct ConstantMethodRefInfo {
    member_info: ConstantMemberRefInfo,
}

impl ConstantInfo for ConstantMethodRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.member_info.read_info(reader);
    }

    fn tag(&self) -> u8 {
        consts::CONSTANT_METHOD_REF
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ConstantMethodRefInfo {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        ConstantMethodRefInfo {
            member_info: ConstantMemberRefInfo::new(cp),
        }
    }

    pub fn class_name(&self) -> String {
        self.member_info
            .constant_pool
            .borrow()
            .get_class_name(self.member_info.class_index)
    }

    pub fn name_and_descriptor(&self) -> (String, String) {
        self.member_info
            .constant_pool
            .borrow()
            .get_name_and_type(self.member_info.name_and_type_index as usize)
    }
}

/// CONSTANT_InterfaceMethod_Ref_info {
///     u1 tag;
///     u2 class_index;
///     u2 name_and_type_index;
/// }
pub struct ConstantInterfaceMethodRefInfo {
    member_info: ConstantMemberRefInfo,
}

impl ConstantInfo for ConstantInterfaceMethodRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.member_info.read_info(reader);
    }

    fn tag(&self) -> u8 {
        consts::CONSTANT_INTERFACE_METHOD_REF
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ConstantInterfaceMethodRefInfo {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        ConstantInterfaceMethodRefInfo {
            member_info: ConstantMemberRefInfo::new(cp),
        }
    }

    pub fn class_name(&self) -> String {
        self.member_info
            .constant_pool
            .borrow()
            .get_class_name(self.member_info.class_index)
    }

    pub fn name_and_descriptor(&self) -> (String, String) {
        self.member_info
            .constant_pool
            .borrow()
            .get_name_and_type(self.member_info.name_and_type_index as usize)
    }
}
