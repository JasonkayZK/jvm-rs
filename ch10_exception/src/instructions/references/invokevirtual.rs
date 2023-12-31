#![allow(non_camel_case_types)]

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::{invoke_method, Instruction};
use crate::instructions::errors::InstructionError;
use crate::rtda::frame::Frame;
use crate::rtda::heap::method_lookup::lookup_method_in_class;
use crate::rtda::heap::method_ref::MethodRef;
use crate::rtda::heap::string_pool;
use crate::rtda::operand_stack::OperandStack;

/// Invoke instance method; dispatch based on class
#[derive(Default, Debug)]
pub struct INVOKE_VIRTUAL {
    index: u64,
}

impl Instruction for INVOKE_VIRTUAL {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_class = frame.method().borrow().get_class();
        let r_cp = current_class.borrow().constant_pool();
        let resolved_method = r_cp
            .borrow_mut()
            .get_constant_mut(self.index as usize)
            .as_any_mut()
            .downcast_mut::<MethodRef>()
            .unwrap()
            .resolved_method(current_class.clone());

        if resolved_method.borrow().is_static() {
            panic!("{}", InstructionError::IncompatibleClassChangeError);
        }

        let _ref = frame
            .operand_stack_mut()
            .get_ref_from_top((resolved_method.borrow().arg_object_ref_count() - 1) as usize);
        if _ref.is_none() {
            if resolved_method.borrow().name() == "println" {
                // Hack!
                println(
                    frame.operand_stack_mut(),
                    resolved_method.borrow().descriptor(),
                );
                return;
            }

            panic!("{}", InstructionError::NullPointerException);
        }

        if resolved_method.borrow().is_protected()
            && resolved_method
                .borrow()
                .get_class()
                .borrow()
                .is_sub_class_of(&current_class)
            && resolved_method
                .borrow()
                .get_class()
                .borrow()
                .get_package_name()
                != current_class.borrow().get_package_name()
            && _ref.as_ref().unwrap().borrow().class().ne(&current_class)
            && !_ref
                .as_ref()
                .unwrap()
                .borrow()
                .class()
                .borrow()
                .is_sub_class_of(&current_class)
        {
            panic!("{}", InstructionError::IllegalAccessError);
        }

        let method_to_be_invoked = lookup_method_in_class(
            _ref.as_ref().unwrap().borrow().class(),
            resolved_method.borrow().name(),
            resolved_method.borrow().descriptor(),
        );

        if method_to_be_invoked.is_none()
            || method_to_be_invoked
                .as_ref()
                .unwrap()
                .borrow()
                .is_abstract()
        {
            panic!("{}", InstructionError::AbstractMethodError);
        }

        invoke_method(frame, method_to_be_invoked.as_ref().unwrap());
    }
}

/// Hack!
fn println(stack: &mut OperandStack, descriptor: String) {
    if descriptor == "(Z)V" {
        println!("{}", stack.pop_int() != 0);
    } else if descriptor == "(C)V"
        || descriptor == "(I)V"
        || descriptor == "(B)V"
        || descriptor == "(S)V"
    {
        println!("{}", stack.pop_int());
    } else if descriptor == "(F)V" {
        println!("{}", stack.pop_float());
    } else if descriptor == "(J)V" {
        println!("{}", stack.pop_long());
    } else if descriptor == "(D)V" {
        println!("{}", stack.pop_double());
    } else if descriptor == "(Ljava/lang/String;)V" {
        let j_str = stack.pop_ref();
        let r_str = string_pool::rust_string(j_str.as_ref().unwrap());
        println!("{}", r_str);
    } else {
        panic!("println: {}", descriptor);
    }
    stack.pop_ref();
}
