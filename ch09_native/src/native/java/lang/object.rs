use std::collections::HashMap;

use log::debug;

use crate::native::errors::NativeMethodError;
use crate::native::registry::{NativeMethod, NativeRegistry};
use crate::rtda::frame::Frame;
use crate::rtda::heap::consts::{CLONEABLE_CLASS, OBJECT_CLASS};

pub(crate) fn init(data: &mut HashMap<String, NativeMethod>) {
    NativeRegistry::_register(
        data,
        OBJECT_CLASS,
        "getClass",
        "()Ljava/lang/Class;",
        get_class,
    );

    NativeRegistry::_register(data, OBJECT_CLASS, "hashCode", "()I", hash_code);

    NativeRegistry::_register(data, OBJECT_CLASS, "clone", "()Ljava/lang/Object;", clone);

    debug!("NativeRegistry: object native method registered!")
}

/// public final native Class<?> getClass();
/// ()Ljava/lang/Class;
fn get_class(frame: &mut Frame) {
    let this = frame.local_vars_mut().get_this();
    let class = this.unwrap().borrow().class().borrow().j_class();
    frame.operand_stack_mut().push_ref(class);
}

/// public native int hashCode();
/// ()I
fn hash_code(frame: &mut Frame) {
    let this = frame.local_vars_mut().get_this();
    let hash = this.unwrap().as_ptr() as i32;
    frame.operand_stack_mut().push_int(hash);
}

/// protected native Object clone();
/// ()Ljava/lang/Object;
fn clone(frame: &mut Frame) {
    let this = frame.local_vars_mut().get_this().unwrap();
    let this_ref = this.borrow();
    let class = this_ref.class();
    let loader = class.borrow().loader().unwrap();
    let cloneable = loader
        .borrow_mut()
        .load_class(loader.clone(), CLONEABLE_CLASS.to_string());
    if !class.borrow().is_implements(&cloneable) {
        panic!("{}", NativeMethodError::CloneNotSupportedException);
    }

    frame
        .operand_stack_mut()
        .push_ref(this.borrow().clone_object(&this.clone()));
}
