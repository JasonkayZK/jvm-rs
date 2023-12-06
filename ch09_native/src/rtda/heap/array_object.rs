use crate::rtda::heap::consts::ArrayTypeEnum::{
    Bytes, Chars, Doubles, Floats, Ints, Longs, Refs, Shorts,
};
use crate::rtda::heap::errors::RuntimeHeapError;
use crate::rtda::object::{Object, ObjectData};
use crate::types::OptionRcRefCell;

/// Byte array
impl ObjectData for Vec<i8> {
    fn tag(&self) -> u8 {
        Bytes.into()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Short array
impl ObjectData for Vec<i16> {
    fn tag(&self) -> u8 {
        u8::from(Shorts)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Int array
impl ObjectData for Vec<i32> {
    fn tag(&self) -> u8 {
        u8::from(Ints)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Long array
impl ObjectData for Vec<i64> {
    fn tag(&self) -> u8 {
        u8::from(Longs)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Char array
impl ObjectData for Vec<u16> {
    fn tag(&self) -> u8 {
        u8::from(Chars)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Float array
impl ObjectData for Vec<f32> {
    fn tag(&self) -> u8 {
        u8::from(Floats)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Double array
impl ObjectData for Vec<f64> {
    fn tag(&self) -> u8 {
        u8::from(Doubles)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Ref array
impl ObjectData for Vec<OptionRcRefCell<Object>> {
    fn tag(&self) -> u8 {
        u8::from(Refs)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Object {
    pub fn bytes_mut(&mut self) -> &mut Vec<i8> {
        self.data_mut()
            .as_any_mut()
            .downcast_mut::<Vec<i8>>()
            .unwrap()
    }

    pub fn shorts_mut(&mut self) -> &mut Vec<i16> {
        self.data_mut()
            .as_any_mut()
            .downcast_mut::<Vec<i16>>()
            .unwrap()
    }

    pub fn ints_mut(&mut self) -> &mut Vec<i32> {
        self.data_mut()
            .as_any_mut()
            .downcast_mut::<Vec<i32>>()
            .unwrap()
    }

    pub fn longs_mut(&mut self) -> &mut Vec<i64> {
        self.data_mut()
            .as_any_mut()
            .downcast_mut::<Vec<i64>>()
            .unwrap()
    }

    pub fn chars_mut(&mut self) -> &mut Vec<u16> {
        self.data_mut()
            .as_any_mut()
            .downcast_mut::<Vec<u16>>()
            .unwrap()
    }

    pub fn floats_mut(&mut self) -> &mut Vec<f32> {
        self.data_mut()
            .as_any_mut()
            .downcast_mut::<Vec<f32>>()
            .unwrap()
    }

    pub fn doubles_mut(&mut self) -> &mut Vec<f64> {
        self.data_mut()
            .as_any_mut()
            .downcast_mut::<Vec<f64>>()
            .unwrap()
    }

    pub fn refs_mut(&mut self) -> &mut Vec<OptionRcRefCell<Object>> {
        self.data_mut()
            .as_any_mut()
            .downcast_mut::<Vec<OptionRcRefCell<Object>>>()
            .unwrap()
    }

    pub fn array_length(&self) -> usize {
        match self.data().tag().into() {
            Bytes => self
                .data()
                .as_any()
                .downcast_ref::<Vec<i8>>()
                .unwrap()
                .len(),
            Shorts => self
                .data()
                .as_any()
                .downcast_ref::<Vec<i16>>()
                .unwrap()
                .len(),
            Ints => self
                .data()
                .as_any()
                .downcast_ref::<Vec<i32>>()
                .unwrap()
                .len(),
            Longs => self
                .data()
                .as_any()
                .downcast_ref::<Vec<i64>>()
                .unwrap()
                .len(),
            Chars => self
                .data()
                .as_any()
                .downcast_ref::<Vec<u16>>()
                .unwrap()
                .len(),
            Floats => self
                .data()
                .as_any()
                .downcast_ref::<Vec<f32>>()
                .unwrap()
                .len(),
            Doubles => self
                .data()
                .as_any()
                .downcast_ref::<Vec<f64>>()
                .unwrap()
                .len(),
            Refs => self
                .data()
                .as_any()
                .downcast_ref::<Vec<OptionRcRefCell<Object>>>()
                .unwrap()
                .len(),
            _ => panic!("{}", RuntimeHeapError::UnknownArrayType(self.data().tag())),
        }
    }
}
