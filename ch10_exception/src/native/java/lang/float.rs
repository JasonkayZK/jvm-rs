use std::collections::HashMap;

use log::debug;

use crate::native::registry::{NativeMethod, NativeRegistry};
use crate::rtda::frame::Frame;
use crate::rtda::heap::consts::FLOAT_CLASS;

pub(crate) fn init(data: &mut HashMap<String, NativeMethod>) {
    NativeRegistry::_register(
        data,
        FLOAT_CLASS,
        "floatToRawIntBits",
        "(F)I",
        float_to_raw_int_bits,
    );
    NativeRegistry::_register(
        data,
        FLOAT_CLASS,
        "intBitsToFloat",
        "(I)F",
        int_bits_to_float,
    );

    debug!("NativeRegistry: float native method registered!")
}

/// public static native int floatToRawIntBits(float value);
/// (F)I
fn float_to_raw_int_bits(frame: &mut Frame) {
    let value = frame.local_vars_mut().get_float(0);
    let bits = f32::to_bits(value);
    frame.operand_stack_mut().push_int(bits as i32);
}

/// public static native float intBitsToFloat(int bits);
/// (I)F
fn int_bits_to_float(frame: &mut Frame) {
    let bits = frame.local_vars_mut().get_int(0);
    let value = f32::from_bits(bits as u32);
    frame.operand_stack_mut().push_float(value);
}
