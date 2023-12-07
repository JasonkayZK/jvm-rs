use std::collections::HashMap;

use log::debug;

use crate::native::errors::NativeMethodError;
use crate::native::registry::{NativeMethod, NativeRegistry};
use crate::rtda::frame::Frame;
use crate::rtda::heap::consts::SYSTEM_CLASS;
use crate::rtda::object::Object;
use crate::types::RcRefCell;

pub(crate) fn init(data: &mut HashMap<String, NativeMethod>) {
    NativeRegistry::_register(
        data,
        SYSTEM_CLASS,
        "arraycopy",
        "(Ljava/lang/Object;ILjava/lang/Object;II)V",
        arraycopy,
    );

    debug!("NativeRegistry: system native method registered!")
}

/// public static native void arraycopy(Object src, int srcPos, Object dest, int destPos, int length)
/// (Ljava/lang/Object;ILjava/lang/Object;II)V
fn arraycopy(frame: &mut Frame) {
    let vars = frame.local_vars_mut();
    let src = vars.get_ref(0);
    let src_pos = vars.get_int(1);
    let dest = vars.get_ref(2);
    let dest_pos = vars.get_int(3);
    let length = vars.get_int(4);

    if src.is_none() || dest.is_none() {
        panic!("{}", NativeMethodError::NullPointerException);
    }

    if !check_arraycopy(src.as_ref().unwrap(), dest.as_ref().unwrap()) {
        panic!("{}", NativeMethodError::ArrayStoreException);
    }

    if src_pos < 0
        || dest_pos < 0
        || length < 0
        || src_pos + length > src.as_ref().unwrap().borrow().array_length() as i32
        || dest_pos + length > dest.as_ref().unwrap().borrow().array_length() as i32
    {
        panic!("{}", NativeMethodError::IndexOutOfBoundsException);
    }

    Object::array_copy(
        src.unwrap(),
        dest.unwrap(),
        src_pos as usize,
        dest_pos as usize,
        length as usize,
    );
}

fn check_arraycopy(src: &RcRefCell<Object>, dest: &RcRefCell<Object>) -> bool {
    let src_class = unsafe { src.borrow().class().as_ptr().as_mut().unwrap() };
    let dest_class = unsafe { dest.borrow().class().as_ptr().as_mut().unwrap() };

    if !src_class.is_array() || !dest_class.is_array() {
        return false;
    }

    if src_class.component_class().borrow().is_primitive()
        || dest_class.component_class().borrow().is_primitive()
    {
        return src_class.eq(&dest_class);
    }

    true
}
