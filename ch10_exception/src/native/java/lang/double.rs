use std::collections::HashMap;

use log::debug;

use crate::native::registry::{NativeMethod, NativeRegistry};
use crate::rtda::frame::Frame;
use crate::rtda::heap::consts::DOUBLE_CLASS;

pub(crate) fn init(data: &mut HashMap<String, NativeMethod>) {
    NativeRegistry::_register(
        data,
        DOUBLE_CLASS,
        "doubleToRawLongBits",
        "(D)J",
        double_to_raw_long_bits,
    );
    NativeRegistry::_register(
        data,
        DOUBLE_CLASS,
        "longBitsToDouble",
        "(J)D",
        long_bits_to_double,
    );

    debug!("NativeRegistry: double native method registered!")
}

/// public static native long doubleToRawLongBits(double value);
/// (D)J
fn double_to_raw_long_bits(frame: &mut Frame) {
    let value = frame.local_vars_mut().get_double(0);
    let bits = f64::to_bits(value);
    frame.operand_stack_mut().push_long(bits as i64);
}

/// public static native double longBitsToDouble(long bits);
/// (J)D
fn long_bits_to_double(frame: &mut Frame) {
    let bits = frame.local_vars_mut().get_long(0);
    let value = f64::from_bits(bits as u64);
    frame.operand_stack_mut().push_double(value);
}
