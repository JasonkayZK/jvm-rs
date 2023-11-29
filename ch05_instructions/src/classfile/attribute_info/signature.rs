//! Signature attribute definition
//!
//! Signature_attribute {
//!     u2 attribute_name_index;
//!     u4 attribute_length;
//!     u2 signature_index;
//! }

use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use super::ConstantPool;
use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct SignatureAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    signature_index: u16,
}

impl Display for SignatureAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[SignatureAttribute]:\n\t{}",
            self.constant_pool.borrow().get_utf8(self.signature_index)
        )
    }
}

impl AttributeInfo for SignatureAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.signature_index = reader.read_u16();
    }
}

impl SignatureAttribute {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        Self {
            constant_pool: cp,
            ..Default::default()
        }
    }
}
