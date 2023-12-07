use std::collections::HashMap;

use log::debug;

use crate::instructions::base::invoke_method;
use crate::native::registry::{NativeMethod, NativeRegistry};
use crate::rtda::frame::Frame;
use crate::rtda::heap::consts::VM_CLASS;
use crate::rtda::heap::string_pool::StringPool;

pub(crate) fn init(data: &mut HashMap<String, NativeMethod>) {
    NativeRegistry::_register(data, VM_CLASS, "initialize", "()V", initialize);

    debug!("NativeRegistry: vm native method registered!")
}

/// Hack implementation for initialize
///
/// For hack VM.getSavedProperty: VM.savedProps.setProperty("foo", "bar")
///
/// private static native void initialize();
/// ()V
fn initialize(frame: &mut Frame) {
    let vm_class = frame.method().borrow().get_class();
    let loader = vm_class.borrow().loader().unwrap();
    let saved_props = vm_class
        .borrow()
        .get_ref_var(
            "savedProps".to_string(),
            "Ljava/util/Properties;".to_string(),
        )
        .unwrap();
    let mut string_pool = StringPool::global().lock().unwrap();
    let key = string_pool.jstring(loader.clone(), "foo".to_string());
    let val = string_pool.jstring(loader.clone(), "bar".to_string());

    let mut operand_stack = frame.operand_stack_mut();
    operand_stack.push_ref(Some(saved_props));
    operand_stack.push_ref(Some(key));
    operand_stack.push_ref(Some(val));

    let props_class = loader
        .borrow_mut()
        .load_class(loader.clone(), "java/util/Properties".to_string());
    let set_prop_method = props_class
        .borrow()
        .get_instance_method(
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
        )
        .unwrap();
    invoke_method(frame, &set_prop_method);
}
