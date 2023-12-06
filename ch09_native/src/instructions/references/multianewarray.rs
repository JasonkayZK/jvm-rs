#![allow(non_camel_case_types)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::instructions::errors::InstructionError;
use crate::rtda::frame::Frame;
use crate::rtda::heap::class::Class;
use crate::rtda::heap::class_ref::ClassRef;
use crate::rtda::object::Object;
use crate::rtda::operand_stack::OperandStack;
use crate::types::RcRefCell;

/// Create new multidimensional array
#[derive(Default, Debug)]
pub struct MULTI_ANEW_ARRAY {
    index: u16,
    dimensions: u8,
}

impl Instruction for MULTI_ANEW_ARRAY {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16();
        self.dimensions = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let method = frame.method();
        let current_class = method.borrow().get_class();
        let r_cp = current_class.borrow_mut().constant_pool();
        let arr_class = r_cp
            .borrow_mut()
            .get_constant_mut(self.index as usize)
            .as_any_mut()
            .downcast_mut::<ClassRef>()
            .unwrap()
            .resolved_class(current_class);

        let stack = frame.operand_stack_mut();
        let counts = pop_and_check_counts(stack, self.dimensions as usize);
        let arr = new_multi_dimensional_array(counts, &arr_class);

        stack.push_ref(Some(Rc::new(RefCell::new(arr))));
    }
}

fn pop_and_check_counts(stack: &mut OperandStack, dimensions: usize) -> Vec<i32> {
    let mut counts = vec![0; dimensions];

    for i in (0..dimensions).rev() {
        let val = stack.pop_int();
        counts[i] = val;
        if val < 0 {
            panic!("{}", InstructionError::NegativeArraySizeException(val))
        }
    }
    counts
}

fn new_multi_dimensional_array(counts: Vec<i32>, arr_class: &RcRefCell<Class>) -> Object {
    let count = counts.first().unwrap();
    let mut arr = arr_class
        .borrow_mut()
        .new_array(arr_class.clone(), *count as usize);

    if counts.len() > 1 {
        let refs = arr.refs_mut();

        for _ref in refs {
            let obj = new_multi_dimensional_array(
                counts[1..].to_vec(),
                &arr_class.borrow_mut().component_class(),
            );
            *_ref = Some(Rc::new(RefCell::new(obj)));
        }
    }

    arr
}
