use crate::rtda::errors::RuntimeDataAreaError;
use crate::rtda::object::ObjectData;
use crate::types::ObjectRef;

#[derive(Clone)]
pub enum HeapObjectRef {
    Num(i32),
    Ref(ObjectRef),
}

#[derive(Clone)]
pub struct HeapObjectRefs {
    refs: Vec<HeapObjectRef>,
}

impl HeapObjectRefs {
    pub fn new(max_locals: usize) -> Self {
        HeapObjectRefs {
            refs: vec![HeapObjectRef::Ref(None); max_locals],
        }
    }

    pub fn set_int(&mut self, index: usize, val: i32) {
        self.refs[index] = HeapObjectRef::Num(val);
    }

    pub fn get_int(&self, index: usize) -> i32 {
        match self.refs[index] {
            HeapObjectRef::Num(val) => val,
            HeapObjectRef::Ref(_) => 0,
        }
    }

    pub fn set_float(&mut self, index: usize, val: f32) {
        let bytes = f32::to_be_bytes(val);
        self.refs[index] = HeapObjectRef::Num(i32::from_be_bytes(bytes));
    }

    pub fn get_float(&self, index: usize) -> f32 {
        match self.refs[index] {
            HeapObjectRef::Num(num) => {
                let bytes = i32::to_be_bytes(num);
                f32::from_be_bytes(bytes)
            }
            HeapObjectRef::Ref(_) => 0.0,
        }
    }

    pub fn set_long(&mut self, index: usize, val: i64) {
        // Long consumes two references
        self.refs[index] = HeapObjectRef::Num(val as i32);
        self.refs[index + 1] = HeapObjectRef::Num((val >> 32) as i32);
    }

    pub fn get_long(&self, index: usize) -> i64 {
        let low = if let HeapObjectRef::Num(low) = self.refs[index] {
            low as u32
        } else {
            0
        };
        let high = if let HeapObjectRef::Num(high) = self.refs[index + 1] {
            high as u32
        } else {
            0
        };
        (high as i64) << 32 | low as i64
    }

    pub fn set_double(&mut self, index: usize, val: f64) {
        // Double consumes two references
        let bytes = f64::to_be_bytes(val);
        let val = i64::from_be_bytes(bytes);
        self.set_long(index, val);
    }

    pub fn get_double(&self, index: usize) -> f64 {
        let bytes = i64::to_be_bytes(self.get_long(index));
        f64::from_be_bytes(bytes)
    }

    pub fn set_ref(&mut self, index: usize, obj_ref: ObjectRef) {
        self.refs[index] = HeapObjectRef::Ref(obj_ref);
    }

    pub fn get_ref(&self, index: usize) -> ObjectRef {
        match &self.refs[index] {
            HeapObjectRef::Num(_) => {
                panic!(
                    "{}",
                    RuntimeDataAreaError::WrongVarRefType("Object".to_string(), "Num".to_string())
                )
            }
            HeapObjectRef::Ref(obj_ref) => obj_ref.clone(),
        }
    }
}

impl ObjectData for HeapObjectRefs {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
