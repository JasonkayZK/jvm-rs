use crate::rtda::heap::consts::ArrayTypeEnum::{
    Bytes, Chars, Doubles, Floats, Ints, Longs, Refs, Shorts, Unknown,
};
use crate::rtda::heap::errors::RuntimeHeapError;

pub static OBJECT_CLASS: &str = "java/lang/Object";

pub static DOUBLE_CLASS: &str = "java/lang/Double";

pub static FLOAT_CLASS: &str = "java/lang/Float";

pub static SYSTEM_CLASS: &str = "java/lang/System";

pub static CLONEABLE_CLASS: &str = "java/lang/Cloneable";

pub static SERIALIZABLE_CLASS: &str = "java/io/Serializable";

pub static STRING_CLASS: &str = "java/lang/String";

pub static CLASS_CLASS: &str = "java/lang/Class";

pub static VM_CLASS: &str = "sun/misc/VM";

pub static THROWABLE_CLASS: &str = "java/lang/Throwable";

pub enum ArrayTypeEnum {
    Bytes,
    Shorts,
    Ints,
    Longs,
    Chars,
    Floats,
    Doubles,
    Refs,
    Unknown,
}

impl From<ArrayTypeEnum> for u8 {
    fn from(value: ArrayTypeEnum) -> Self {
        match value {
            Bytes => 1,
            Shorts => 2,
            Ints => 3,
            Longs => 4,
            Chars => 5,
            Floats => 6,
            Doubles => 7,
            Refs => 8,
            Unknown => 0,
        }
    }
}

impl TryFrom<u8> for ArrayTypeEnum {
    type Error = RuntimeHeapError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Bytes),
            2 => Ok(Shorts),
            3 => Ok(Ints),
            4 => Ok(Longs),
            5 => Ok(Chars),
            6 => Ok(Floats),
            7 => Ok(Doubles),
            8 => Ok(Refs),
            _ => Err(RuntimeHeapError::UnknownArrayType(value)),
        }
    }
}
