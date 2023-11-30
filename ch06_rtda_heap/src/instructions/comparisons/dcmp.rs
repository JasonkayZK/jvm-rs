#![allow(non_camel_case_types)]

use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Compare double
#[derive(Default, Debug)]
pub struct DCMPG;

impl Instruction for DCMPG {
    fn execute(&mut self, frame: &mut Frame) {
        _dcmp(frame, true);
    }
}

#[derive(Default, Debug)]
pub struct DCMPL;

impl Instruction for DCMPL {
    fn execute(&mut self, frame: &mut Frame) {
        _dcmp(frame, false);
    }
}

/// Compare double
///
/// Gflag has used when two double values can not be compared
fn _dcmp(frame: &mut Frame, g_flag: bool) {
    let stack = frame.operand_stack_mut();
    let v2 = stack.pop_double();
    let v1 = stack.pop_double();
    if v1 > v2 {
        stack.push_int(1);
    } else if v1 == v2 {
        stack.push_int(0);
    } else if v1 < v2 {
        stack.push_int(-1);
    } else if g_flag {
        stack.push_int(1);
    } else {
        stack.push_int(-1);
    }
}
