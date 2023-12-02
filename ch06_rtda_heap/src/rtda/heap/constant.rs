use crate::classfile::constant_pool::consts::{
    CONSTANT_DOUBLE, CONSTANT_FLOAT, CONSTANT_INTEGER, CONSTANT_LONG, CONSTANT_STRING,
};

pub trait Constant {
    fn tag(&self) -> u8;

    fn as_any(&self) -> &dyn std::any::Any;

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl Constant for i32 {
    fn tag(&self) -> u8 {
        CONSTANT_INTEGER
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Constant for f32 {
    fn tag(&self) -> u8 {
        CONSTANT_FLOAT
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Constant for i64 {
    fn tag(&self) -> u8 {
        CONSTANT_LONG
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Constant for f64 {
    fn tag(&self) -> u8 {
        CONSTANT_DOUBLE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Constant for String {
    fn tag(&self) -> u8 {
        CONSTANT_STRING
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
