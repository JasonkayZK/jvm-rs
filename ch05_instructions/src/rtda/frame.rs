//! Stack Frame implementation
//!
//! A frame is storing the local variable or temporary variables
//!
//! in current function calling status.

use std::cell::RefCell;
use std::rc::Rc;

use crate::rtda::thread::Thread;

use super::{local_var::LocalVar, operand_stack::OperandStack};

/// Stack Frame
#[derive(Debug)]
pub struct Frame {
    local_var: LocalVar,
    operand_stack: OperandStack,
    // The next instruction after the call
    next_pc: i64,
    thread_ref: Rc<RefCell<Thread>>,
}

impl Frame {
    pub fn new(thread: Rc<RefCell<Thread>>, max_locals: usize, max_size: usize) -> Self {
        Frame {
            local_var: LocalVar::new(max_locals),
            operand_stack: OperandStack::new(max_size),
            next_pc: 0,
            thread_ref: thread,
        }
    }

    pub fn local_vars_mut(&mut self) -> &mut LocalVar {
        &mut self.local_var
    }

    pub fn operand_stack_mut(&mut self) -> &mut OperandStack {
        &mut self.operand_stack
    }

    pub fn set_next_pc(&mut self, next_pc: i64) {
        self.next_pc = next_pc;
    }

    pub fn next_pc(&self) -> i64 {
        self.next_pc
    }

    pub fn thread(&self) -> &Rc<RefCell<Thread>> {
        &self.thread_ref
    }
}
