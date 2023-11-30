#![allow(non_camel_case_types)]

use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Compare float
#[derive(Default, Debug)]
pub struct FCMPG;

impl Instruction for FCMPG {
    fn execute(&mut self, frame: &mut Frame) {
        _fcmp(frame, true);
    }
}

#[derive(Default, Debug)]
pub struct FCMPL;

impl Instruction for FCMPL {
    fn execute(&mut self, frame: &mut Frame) {
        _fcmp(frame, false);
    }
}

/// Compare float
///
/// Gflag has used when two float values can not be compared
fn _fcmp(frame: &mut Frame, g_flag: bool) {
    let stack = frame.operand_stack_mut();
    let v2 = stack.pop_float();
    let v1 = stack.pop_float();
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
