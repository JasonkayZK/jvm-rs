//! CONSTANT_Numeric definition
//!
//! Include:
//!
//! - CONSTANT_Integer
//! - CONSTANT_Float
//! - CONSTANT_Long
//! - CONSTANT_Double
//!

use super::{consts, ClassReader, ConstantInfo};

/// CONSTANT_Integer_info {
///     u1 tag;
///     u4 bytes;
/// }
#[derive(Default)]
pub struct ConstantIntegerInfo {
    val: i32,
}

impl ConstantInfo for ConstantIntegerInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.val = reader.read_u32() as i32
    }

    fn tag(&self) -> u8 {
        consts::CONSTANT_INTEGER
    }
}

impl ConstantIntegerInfo {
    pub fn value(&self) -> i32 {
        self.val
    }
}

/// CONSTANT_Float_info {
///     u1 tag;
///     u4 bytes;
/// }
#[derive(Default)]
pub struct ConstantFloatInfo {
    val: f32,
}

impl ConstantInfo for ConstantFloatInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.val = f32::from_bits(reader.read_u32());
    }

    fn tag(&self) -> u8 {
        consts::CONSTANT_FLOAT
    }
}

impl ConstantFloatInfo {
    pub fn value(&self) -> f32 {
        self.val
    }
}

/// CONSTANT_Long_info {
///     u1 tag;
///     u4 high_bytes;
///     u4 low_bytes;
/// }
#[derive(Default)]
pub struct ConstantLongInfo {
    val: i64,
}

impl ConstantInfo for ConstantLongInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.val = reader.read_u64() as i64;
    }

    fn tag(&self) -> u8 {
        consts::CONSTANT_LONG
    }
}

impl ConstantLongInfo {
    pub fn value(&self) -> i64 {
        self.val
    }
}

/// CONSTANT_Double_info {
///     u1 tag;
///     u4 high_bytes;
///     u4 low_bytes;
/// }
#[derive(Default)]
pub struct ConstantDoubleInfo {
    val: f64,
}

impl ConstantInfo for ConstantDoubleInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.val = f64::from_bits(reader.read_u64());
    }

    fn tag(&self) -> u8 {
        consts::CONSTANT_DOUBLE
    }
}

impl ConstantDoubleInfo {
    pub fn value(&self) -> f64 {
        self.val
    }
}
