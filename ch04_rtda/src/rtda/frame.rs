//! Stack Frame implementation
//!
//! A frame is storing the local variable or temporary variables
//!
//! in current function calling status.

use super::{local_var::LocalVar, operand_stack::OperandStack};

pub struct Frame {
    local_var: LocalVar,
    operand_stack: OperandStack,
}

impl Frame {
    pub fn new(max_locals: usize, max_size: usize) -> Self {
        Frame {
            local_var: LocalVar::new(max_locals),
            operand_stack: OperandStack::new(max_size),
        }
    }

    pub fn local_vars_mut(&mut self) -> &mut LocalVar {
        &mut self.local_var
    }

    pub fn operand_stack_mut(&mut self) -> &mut OperandStack {
        &mut self.operand_stack
    }
}
