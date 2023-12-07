use std::collections::HashMap;

use log::debug;

use crate::native::registry::{NativeMethod, NativeRegistry};
use crate::rtda::frame::Frame;
use crate::rtda::heap::consts::CLASS_CLASS;
use crate::rtda::heap::string_pool;
use crate::rtda::heap::string_pool::StringPool;
use crate::rtda::object::ClassData;

pub(crate) fn init(data: &mut HashMap<String, NativeMethod>) {
    NativeRegistry::_register(
        data,
        CLASS_CLASS,
        "getPrimitiveClass",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        get_primitive_class,
    );
    NativeRegistry::_register(
        data,
        CLASS_CLASS,
        "getName0",
        "()Ljava/lang/String;",
        get_name0,
    );
    NativeRegistry::_register(
        data,
        CLASS_CLASS,
        "desiredAssertionStatus0",
        "(Ljava/lang/Class;)Z",
        desired_assertion_status0,
    );

    debug!("NativeRegistry: class native method registered!")
}

/// static native Class<?> getPrimitiveClass(String name);
/// (Ljava/lang/String;)Ljava/lang/Class;
fn get_primitive_class(frame: &mut Frame) {
    let name_obj = frame.local_vars_mut().get_ref(0);
    let name = string_pool::rust_string(name_obj.as_ref().unwrap());

    let current_class = frame.method().borrow().get_class();
    let current_class = unsafe { current_class.as_ptr().as_mut().unwrap() };
    let loader = current_class.loader().unwrap();
    let class = loader
        .borrow_mut()
        .load_class(loader.clone(), name)
        .borrow()
        .j_class();

    frame.operand_stack_mut().push_ref(class);
}

/// private native String getName0();
/// ()Ljava/lang/String;
fn get_name0(frame: &mut Frame) {
    let this = frame.local_vars_mut().get_this();

    let current_class = frame.method().borrow().get_class();
    let current_class = unsafe { current_class.as_ptr().as_mut().unwrap() };
    let loader = current_class.loader().unwrap();

    let name = this
        .unwrap()
        .borrow()
        .extra()
        .unwrap()
        .as_any()
        .downcast_ref::<ClassData>()
        .unwrap()
        .java_name();
    let name_obj = StringPool::global().lock().unwrap().jstring(loader, name);

    frame.operand_stack_mut().push_ref(Some(name_obj));
}

/// private static native boolean desiredAssertionStatus0(Class<?> clazz);
/// (Ljava/lang/Class;)Z
fn desired_assertion_status0(frame: &mut Frame) {
    frame.operand_stack_mut().push_boolean(false);
}
