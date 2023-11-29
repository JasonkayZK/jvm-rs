//! The markers for the attributes
//!
//! Markers have no real values, just like Sync, Send trait in rust.
//!
//! - Deprecated
//! - Synthetic

use super::{AttributeInfo, ClassReader};

/// Deprecated_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
/// }
#[derive(Default)]
pub struct DeprecatedAttribute {}

impl AttributeInfo for DeprecatedAttribute {
    fn read_info(&mut self, _reader: &mut ClassReader) {}
}

/// Synthetic_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
/// }
#[derive(Default)]
pub struct SyntheticAttribute {}

impl AttributeInfo for SyntheticAttribute {
    fn read_info(&mut self, _reader: &mut ClassReader) {}
}

pub struct MarkerAttribute {}

impl AttributeInfo for MarkerAttribute {
    fn read_info(&mut self, _reader: &mut ClassReader) {}
}
