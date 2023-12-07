use std::collections::HashMap;

use log::debug;

use crate::native::registry::{NativeMethod, NativeRegistry};
use crate::rtda::frame::Frame;
use crate::rtda::heap::consts::STRING_CLASS;
use crate::rtda::heap::string_pool;

pub(crate) fn init(data: &mut HashMap<String, NativeMethod>) {
    NativeRegistry::_register(data, STRING_CLASS, "intern", "()Ljava/lang/String;", intern);

    debug!("NativeRegistry: string native method registered!")
}

/// public native String intern();
/// ()Ljava/lang/String;
fn intern(frame: &mut Frame) {
    let this = frame.local_vars_mut().get_this();
    let interned = string_pool::intern_string(this.as_ref().unwrap());

    frame.operand_stack_mut().push_ref(Some(interned));
}
