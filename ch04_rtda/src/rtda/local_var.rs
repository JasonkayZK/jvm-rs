use crate::rtda::errors::RuntimeDataAreaError;
use crate::rtda::types::ObjectRef;

#[derive(Clone)]
pub enum VarRef {
    Num(i32),
    Ref(ObjectRef),
}

pub struct LocalVar {
    vars: Vec<VarRef>,
}

impl LocalVar {
    pub fn new(max_locals: usize) -> Self {
        LocalVar {
            vars: vec![VarRef::Ref(None); max_locals],
        }
    }

    pub fn set_int(&mut self, index: usize, val: i32) {
        self.vars[index] = VarRef::Num(val);
    }

    pub fn get_int(&self, index: usize) -> i32 {
        match self.vars[index] {
            VarRef::Num(val) => val,
            VarRef::Ref(_) => {
                panic!(
                    "{}",
                    RuntimeDataAreaError::WrongVarRefType("Int".to_string(), "Object".to_string())
                )
            }
        }
    }

    pub fn set_float(&mut self, index: usize, val: f32) {
        let bytes = f32::to_be_bytes(val);
        self.vars[index] = VarRef::Num(i32::from_be_bytes(bytes));
    }

    pub fn get_float(&self, index: usize) -> f32 {
        match self.vars[index] {
            VarRef::Num(num) => {
                let bytes = i32::to_be_bytes(num);
                f32::from_be_bytes(bytes)
            }
            VarRef::Ref(_) => {
                panic!(
                    "{}",
                    RuntimeDataAreaError::WrongVarRefType(
                        "Float".to_string(),
                        "Object".to_string()
                    )
                )
            }
        }
    }

    pub fn set_long(&mut self, index: usize, val: i64) {
        // Long consumes two slots
        self.vars[index] = VarRef::Num(val as i32);
        self.vars[index + 1] = VarRef::Num((val >> 32) as i32);
    }

    pub fn get_long(&self, index: usize) -> i64 {
        let low = if let VarRef::Num(low) = self.vars[index] {
            low as u32
        } else {
            panic!(
                "{}",
                RuntimeDataAreaError::WrongVarRefType(
                    "LongLowBit".to_string(),
                    "Object".to_string()
                )
            )
        };
        let high = if let VarRef::Num(high) = self.vars[index + 1] {
            high as u32
        } else {
            panic!(
                "{}",
                RuntimeDataAreaError::WrongVarRefType(
                    "LongHighBit".to_string(),
                    "Object".to_string()
                )
            )
        };
        (high as i64) << 32 | low as i64
    }

    pub fn set_double(&mut self, index: usize, val: f64) {
        // Double consumes two slots
        let bytes = f64::to_be_bytes(val);
        let val = i64::from_be_bytes(bytes);
        self.set_long(index, val);
    }

    pub fn get_double(&self, index: usize) -> f64 {
        let bytes = i64::to_be_bytes(self.get_long(index));
        f64::from_be_bytes(bytes)
    }

    pub fn set_ref(&mut self, index: usize, obj_ref: ObjectRef) {
        self.vars[index] = VarRef::Ref(obj_ref);
    }

    pub fn get_ref(&self, index: usize) -> ObjectRef {
        match self.vars[index] {
            VarRef::Num(_) => {
                panic!(
                    "{}",
                    RuntimeDataAreaError::WrongVarRefType("Object".to_string(), "Num".to_string())
                )
            }
            VarRef::Ref(obj_ref) => obj_ref,
        }
    }
}
