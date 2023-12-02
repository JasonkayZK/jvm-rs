//! JVM Operand Stack implementation with fixed stack size
//!
//! Operand Stack stores the temporary values during the function call

use crate::rtda::local_var::VarRef;
use crate::rtda::object::Object;
use crate::rtda::types::ObjectRef;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct OperandStack {
    size: usize,
    vars: Vec<VarRef>,
}

impl OperandStack {
    pub fn new(max_stack: usize) -> Self {
        OperandStack {
            size: 0,
            vars: vec![VarRef::Ref(None); max_stack],
        }
    }

    pub fn push_int(&mut self, val: i32) {
        self.vars[self.size] = VarRef::Num(val);
        self.size += 1;
    }

    pub fn pop_int(&mut self) -> i32 {
        self.size -= 1;
        match self.vars[self.size] {
            VarRef::Num(val) => val,
            VarRef::Ref(_) => 0,
        }
    }

    pub fn push_float(&mut self, val: f32) {
        let bytes = f32::to_be_bytes(val);
        self.vars[self.size] = VarRef::Num(i32::from_be_bytes(bytes));
        self.size += 1;
    }

    pub fn pop_float(&mut self) -> f32 {
        self.size -= 1;
        match self.vars[self.size] {
            VarRef::Num(num) => {
                let bytes = i32::to_be_bytes(num);
                f32::from_be_bytes(bytes)
            }
            VarRef::Ref(_) => 0.0,
        }
    }

    pub fn push_long(&mut self, val: i64) {
        // Long consumes two references
        self.vars[self.size] = VarRef::Num(val as i32);
        self.vars[self.size + 1] = VarRef::Num((val >> 32) as i32);
        self.size += 2;
    }

    pub fn pop_long(&mut self) -> i64 {
        self.size -= 2;
        let low = if let VarRef::Num(low) = self.vars[self.size] {
            low as u32
        } else {
            0
        };
        let high = if let VarRef::Num(high) = self.vars[self.size + 1] {
            high as u32
        } else {
            0
        };
        (high as i64) << 32 | low as i64
    }

    pub fn push_double(&mut self, val: f64) {
        // Double consumes two references
        let bytes = f64::to_be_bytes(val);
        self.push_long(i64::from_be_bytes(bytes));
    }

    pub fn pop_double(&mut self) -> f64 {
        let bytes = i64::to_be_bytes(self.pop_long());
        f64::from_be_bytes(bytes)
    }

    pub fn push_ref(&mut self, val: ObjectRef) {
        self.vars[self.size] = VarRef::Ref(val);
        self.size += 1;
    }

    pub fn pop_ref(&mut self) -> ObjectRef {
        self.size -= 1;
        match &self.vars[self.size] {
            VarRef::Num(_) => None,
            VarRef::Ref(obj_ref) => obj_ref.clone(),
        }
    }

    pub fn get_ref_from_top(&self, n: usize) -> Option<Rc<RefCell<Object>>> {
        match self.vars[self.size - n - 1].clone() {
            VarRef::Num(_) => None,
            VarRef::Ref(obj_ref) => obj_ref,
        }
    }

    pub fn push_var(&mut self, slot: VarRef) {
        self.vars[self.size] = slot;
        self.size += 1;
    }

    pub fn pop_var(&mut self) -> VarRef {
        self.size -= 1;
        self.vars[self.size].clone()
    }
}
