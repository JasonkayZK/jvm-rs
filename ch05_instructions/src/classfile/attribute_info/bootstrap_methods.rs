//! BootstrapMethod attribute definition
//!
//! BootstrapMethod_attribute {
//! u2 attribute_name_index;
//! u4 attribute_length;
//! u2 num_bootstrap_methods;
//!     {   u2 bootstrap_method_ref;
//!         u2 num_bootstrap_arguments;
//!         u2 bootstrap_arguments[num_bootstrap_arguments];
//!     } bootstrap_methods[num_bootstrap_methods];
//! }

use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::classfile::attribute_info::types::AttributeTypeNameEnum;
use crate::classfile::constant_pool::ConstantPool;

use super::{AttributeInfo, ClassReader};

pub struct BootstrapMethod {
    bootstrap_method_ref: u16,
    bootstrap_arguments: Vec<u16>,
}

#[derive(Default)]
pub struct BootstrapMethodsAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    bootstrap_methods: Vec<BootstrapMethod>,
}

impl BootstrapMethodsAttribute {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> BootstrapMethodsAttribute {
        Self {
            constant_pool: cp,
            ..Default::default()
        }
    }
}

impl Display for BootstrapMethodsAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cp = self.constant_pool.borrow();
        let mut entries = Vec::with_capacity(self.bootstrap_methods.len());
        for method in &self.bootstrap_methods {
            entries.push(format!(
                "[BootstrapMethod]: name: {}, args_len: {}",
                cp.get_utf8(method.bootstrap_method_ref),
                method.bootstrap_arguments.len()
            ))
        }

        write!(
            f,
            "[BootstrapMethodsAttribute]:\n\t{}",
            entries.join("\n\t\t")
        )
    }
}

impl AttributeInfo for BootstrapMethodsAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let num_bootstrap_methods = reader.read_u16();
        let mut bootstrap_methods = vec![];
        for _i in 0..num_bootstrap_methods {
            bootstrap_methods.push(BootstrapMethod {
                bootstrap_method_ref: reader.read_u16(),
                bootstrap_arguments: reader.read_u16s(),
            });
        }
        self.bootstrap_methods = bootstrap_methods;
    }

    fn name(&self) -> &str {
        AttributeTypeNameEnum::BootstrapMethods.into()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
