//! A JVM Stack implementation with fixed stack size
//!
//! for storing local variables in function

use std::cell::RefCell;
use std::rc::Rc;

use crate::rtda::errors::RuntimeDataAreaError;
use crate::rtda::frame::Frame;

#[derive(Default)]
pub struct Stack {
    max_size: usize,
    top: usize,
    frames: Vec<Option<Rc<RefCell<Frame>>>>,
}

impl Stack {
    pub fn new(max_size: usize) -> Self {
        Stack {
            max_size,
            top: 0,
            frames: vec![None; max_size],
        }
    }

    pub fn push(&mut self, frame: Frame) {
        if self.top >= self.max_size {
            panic!("{}", RuntimeDataAreaError::StackOverflow);
        }

        self.frames[self.top] = Some(Rc::new(RefCell::new(frame)));
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<Rc<RefCell<Frame>>> {
        if self.top == 0 {
            panic!("{}", RuntimeDataAreaError::StackEmpty);
        }
        self.top -= 1;
        self.frames[self.top].clone()
    }

    pub fn top(&self) -> Rc<RefCell<Frame>> {
        if self.top == 0 {
            panic!("{}", RuntimeDataAreaError::StackEmpty);
        }
        self.frames[self.top - 1].clone().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }
}
