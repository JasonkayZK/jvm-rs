use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::rtda::heap::errors::RuntimeHeapError;

pub static PRIMITIVE_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut h = HashMap::new();
    h.insert("void", "V");
    h.insert("boolean", "Z");
    h.insert("byte", "B");
    h.insert("short", "S");
    h.insert("int", "I");
    h.insert("long", "J");
    h.insert("char", "C");
    h.insert("float", "F");
    h.insert("double", "D");
    h
});

/// [XXX -> [[XXX
/// int -> [I
/// XXX -> [LXXX;
pub fn get_array_class_name(class_name: &str) -> String {
    String::from("[") + &to_descriptor(class_name)
}

/// [[XXX -> [XXX
/// [LXXX; -> XXX
/// [I -> int
pub fn get_component_class_name(class_name: &str) -> String {
    if class_name.as_bytes()[0] == b'[' {
        let component_type_descriptor = class_name[1..].into();
        return to_class_name(component_type_descriptor);
    }
    panic!(
        "{}",
        RuntimeHeapError::NotArrayClass(class_name.to_string())
    );
}

/// [XXX => [XXX
/// int  => I
/// XXX  => LXXX;
fn to_descriptor(class_name: &str) -> String {
    if class_name.as_bytes()[0] == b'[' {
        // Array
        return class_name.to_string();
    }
    if let Some(v) = PRIMITIVE_TYPES.get(class_name) {
        // Primitive
        return String::from(*v);
    }
    // Object
    String::from("L") + class_name + ";"
}

/// [XXX  => [XXX
/// LXXX; => XXX
/// I     => int
fn to_class_name(descriptor: String) -> String {
    let d = descriptor.as_bytes()[0];
    if d == b'[' {
        // Array Class
        return descriptor;
    }
    if d == b'L' {
        let strs = descriptor.as_str();
        // Object
        return strs[1..strs.len() - 1].into();
    }
    for (class_name, desc) in PRIMITIVE_TYPES.iter() {
        if descriptor == *desc {
            // Primitive
            return (*class_name).into();
        }
    }
    panic!("Invalid descriptor: {}", descriptor)
}
