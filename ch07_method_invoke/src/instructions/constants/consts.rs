//! CONST instructions implementation
//!
//! CONST instruction push the constant value to the stack
//!
//! List:
//!  - ACONST_NULL
//!  - DCONST_0
//!  - DCONST_1
//!  - FCONST_0
//!  - FCONST_1
//!  - FCONST_2
//!  - ICONST_M1
//!  - ICONST_0
//!  - ICONST_1
//!  - ICONST_2
//!  - ICONST_3
//!  - ICONST_4
//!  - ICONST_5
//!  - LCONST_0
//!  - LCONST_1
//!
#![allow(non_camel_case_types)]

use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

#[derive(Default, Debug)]
pub struct ACONST_NULL;

/// Push NULL constant
impl Instruction for ACONST_NULL {
    fn execute(&mut self, frame: &mut Frame) {
        // Push NULL
        frame.operand_stack_mut().push_ref(None);
    }
}

/// Push double constant 0.0
#[derive(Default, Debug)]
pub struct DCONST_0;

impl Instruction for DCONST_0 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_double(0.0);
    }
}

/// Push double constant 1.0
#[derive(Default, Debug)]
pub struct DCONST_1;

impl Instruction for DCONST_1 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_double(1.0);
    }
}

/// Push float constant 0.0
#[derive(Default, Debug)]
pub struct FCONST_0;

impl Instruction for FCONST_0 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_float(0.0);
    }
}

/// Push float constant 1.0
#[derive(Default, Debug)]
pub struct FCONST_1;

impl Instruction for FCONST_1 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_float(1.0);
    }
}

/// Push float constant 2.0
#[derive(Default, Debug)]
pub struct FCONST_2;

impl Instruction for FCONST_2 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_float(2.0);
    }
}

/// Push int constant -1
#[derive(Default, Debug)]
pub struct ICONST_M1;

impl Instruction for ICONST_M1 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_int(-1);
    }
}

/// Push int constant 0
#[derive(Default, Debug)]
pub struct ICONST_0;

impl Instruction for ICONST_0 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_int(0);
    }
}

/// Push int constant 1
#[derive(Default, Debug)]
pub struct ICONST_1;

impl Instruction for ICONST_1 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_int(1);
    }
}

/// Push int constant 2
#[derive(Default, Debug)]
pub struct ICONST_2;

impl Instruction for ICONST_2 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_int(2);
    }
}

/// Push int constant 3
#[derive(Default, Debug)]
pub struct ICONST_3;

impl Instruction for ICONST_3 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_int(3);
    }
}

/// Push int constant 4
#[derive(Default, Debug)]
pub struct ICONST_4;

impl Instruction for ICONST_4 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_int(4);
    }
}

/// Push int constant 5
#[derive(Default, Debug)]
pub struct ICONST_5;

impl Instruction for ICONST_5 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_int(5);
    }
}

/// Push long constant 0L
#[derive(Default, Debug)]
pub struct LCONST_0;

impl Instruction for LCONST_0 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_long(0);
    }
}

/// Push long constant 1L
#[derive(Default, Debug)]
pub struct LCONST_1;

impl Instruction for LCONST_1 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack_mut().push_long(1);
    }
}
