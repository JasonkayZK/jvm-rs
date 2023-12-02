use std::cell::RefCell;
use std::rc::Rc;

use crate::rtda::frame::Frame;
use crate::rtda::heap::method::Method;
use crate::rtda::stack::Stack;
use crate::types::RcRefCell;

const MAX_RUNTIME_STACK_SIZE: usize = 1024;

#[derive(Default)]
pub struct Thread {
    pc: i64,
    stack: Stack,
}

impl Thread {
    pub fn new() -> Self {
        Thread {
            stack: Stack::new(MAX_RUNTIME_STACK_SIZE),
            ..Default::default()
        }
    }

    pub fn new_ref() -> Rc<RefCell<Thread>> {
        Rc::new(RefCell::new(Self::new()))
    }

    pub fn pc(&self) -> i64 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: i64) {
        self.pc = pc;
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.stack.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<Rc<RefCell<Frame>>> {
        self.stack.pop()
    }

    pub fn current_frame(&self) -> Rc<RefCell<Frame>> {
        self.stack.top()
    }

    pub fn new_frame(thread_ref: Rc<RefCell<Thread>>, method: RcRefCell<Method>) -> Frame {
        Frame::new(thread_ref, method)
    }
}
