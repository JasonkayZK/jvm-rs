use crate::rtda::heap::consts::ArrayTypeEnum::{
    Bytes, Chars, Doubles, Floats, Ints, Longs, Refs, Shorts, Unknown,
};
use crate::rtda::heap::errors::RuntimeHeapError;

pub static OBJECT_CLASS: &str = "java/lang/Object";

pub static CLONEABLE_CLASS: &str = "java/lang/Cloneable";

pub static SERIALIZABLE_CLASS: &str = "java/io/Serializable";

pub static STRING_CLASS: &str = "java/lang/String";

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

impl From<u8> for ArrayTypeEnum {
    fn from(value: u8) -> Self {
        match value {
            1 => Bytes,
            2 => Shorts,
            3 => Ints,
            4 => Longs,
            5 => Chars,
            6 => Floats,
            7 => Doubles,
            8 => Refs,
            _ => panic!("{}", RuntimeHeapError::UnknownArrayType(value)),
        }
    }
}
