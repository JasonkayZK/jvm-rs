use crate::classfile::class_reader::ClassReader;

pub trait ConstantInfo {
    /// Read next info from reader
    fn read_info(&mut self, reader: &mut ClassReader);

    /// Get tag
    fn tag(&self) -> u8;
}
