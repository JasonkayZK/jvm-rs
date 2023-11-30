#![allow(non_camel_case_types)]

use crate::instructions::base::Instruction;
use crate::rtda::frame::Frame;

/// Duplicate the top operand stack value
#[derive(Default, Debug)]
pub struct DUP;

impl Instruction for DUP {
    /// bottom -> top
    /// [...][c][b][a]
    ///             \ _
    ///                |
    ///                V
    /// [...][c][b][a][a]
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let reference = stack.pop_ref();
        let reference2 = reference.clone();
        stack.push_ref(reference);
        stack.push_ref(reference2);
    }
}

/// Duplicate the top operand stack value and insert two values down
#[derive(Default, Debug)]
pub struct DUP_X1;

impl Instruction for DUP_X1 {
    /// bottom -> top
    /// [...][c][b][a]
    ///           _/
    ///          |
    ///          V
    /// [...][c][a][b][a]
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let reference1 = stack.pop_ref();
        let reference2 = stack.pop_ref();
        stack.push_ref(reference1.clone());
        stack.push_ref(reference2);
        stack.push_ref(reference1);
    }
}

/// Duplicate the top operand stack value and insert two or three values down
#[derive(Default, Debug)]
pub struct DUP_X2;

impl Instruction for DUP_X2 {
    /// bottom -> top
    /// [...][c][b][a]
    ///        ____/
    ///       |
    ///       V
    /// [...][a][c][b][a]
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let reference1 = stack.pop_ref();
        let reference2 = stack.pop_ref();
        let reference3 = stack.pop_ref();
        stack.push_ref(reference1.clone());
        stack.push_ref(reference3);
        stack.push_ref(reference2);
        stack.push_ref(reference1);
    }
}

/// Duplicate the top one or two operand stack values
#[derive(Default, Debug)]
pub struct DUP2;

impl Instruction for DUP2 {
    /// bottom -> top
    /// [...][c][b][a]_____
    ///           \_____  |
    ///                |  |
    ///                V  V
    /// [...][c][b][a][b][a]
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let reference1 = stack.pop_ref();
        let reference2 = stack.pop_ref();
        stack.push_ref(reference2.clone());
        stack.push_ref(reference1.clone());
        stack.push_ref(reference2);
        stack.push_ref(reference1);
    }
}

/// Duplicate the top one or two operand stack values and insert two or three values down
#[derive(Default, Debug)]
pub struct DUP2_X1;

impl Instruction for DUP2_X1 {
    /// bottom -> top
    /// [...][c][b][a]
    ///        _/ _/
    ///       |  |
    ///       V  V
    /// [...][b][a][c][b][a]
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let reference1 = stack.pop_ref();
        let reference2 = stack.pop_ref();
        let reference3 = stack.pop_ref();
        stack.push_ref(reference2.clone());
        stack.push_ref(reference1.clone());
        stack.push_ref(reference3);
        stack.push_ref(reference2);
        stack.push_ref(reference1);
    }
}

/// Duplicate the top one or two operand stack values and insert two, three, or four values down
#[derive(Default, Debug)]
pub struct DUP2_X2;

impl Instruction for DUP2_X2 {
    /// bottom -> top
    /// [...][d][c][b][a]
    ///        ____/ __/
    ///       |   __/
    ///       V  V
    /// [...][b][a][d][c][b][a]
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack_mut();
        let reference1 = stack.pop_ref();
        let reference2 = stack.pop_ref();
        let reference3 = stack.pop_ref();
        let reference4 = stack.pop_ref();
        stack.push_ref(reference2.clone());
        stack.push_ref(reference1.clone());
        stack.push_ref(reference4);
        stack.push_ref(reference3);
        stack.push_ref(reference2);
        stack.push_ref(reference1);
    }
}
