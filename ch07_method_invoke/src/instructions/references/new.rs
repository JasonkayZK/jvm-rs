#![allow(non_camel_case_types)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;
use crate::rtda::heap::class_ref::ClassRef;
use crate::rtda::object::Object;

/// Create new object
#[derive(Default, Debug)]
pub struct NEW {
    index: u64,
}

impl Instruction for NEW {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let method = frame.method();
        let current_class = method.borrow().get_class();
        let r_cp = current_class.borrow_mut().constant_pool();
        let class = r_cp
            .borrow_mut()
            .get_constant_mut(self.index as usize)
            .as_any_mut()
            .downcast_mut::<ClassRef>()
            .unwrap()
            .resolved_class(current_class);

        // TODO: init class

        if class.borrow().is_interface() || class.borrow().is_abstract() {
            panic!("java.lang.InstantiationError");
        }
        let mut _ref = Object::new(class);
        frame
            .operand_stack_mut()
            .push_ref(Some(Rc::new(RefCell::new(_ref))));
    }
}
