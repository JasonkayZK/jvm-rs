use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

use crate::native::{class, double, float, object, string, system, throwable, vm};
use crate::rtda::frame::Frame;

/// The native method definition
///
/// Note that there is no return value, the return value has been stored
///
/// on the stack.
pub type NativeMethod = fn(frame: &mut Frame);

/// An empty native method
const EMPTY_NATIVE_METHOD: NativeMethod = |frame: &mut Frame| {
    // Do nothing
};

/// Native method registry stores all registered native methods in JVM.
///
/// Native methods call will firstly found the entrance here.
pub struct NativeRegistry {
    data: HashMap<String, NativeMethod>,
}

impl NativeRegistry {
    pub fn global() -> &'static RwLock<NativeRegistry> {
        static REGISTRY: OnceLock<RwLock<NativeRegistry>> = OnceLock::new();
        REGISTRY.get_or_init(|| {
            let mut data = HashMap::new();
            Self::init(&mut data);
            RwLock::new(NativeRegistry { data })
        })
    }

    pub fn init(data: &mut HashMap<String, NativeMethod>) {
        class::init(data);
        object::init(data);
        double::init(data);
        float::init(data);
        string::init(data);
        system::init(data);
        vm::init(data);
        throwable::init(data);
    }

    pub fn register(
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
        method: NativeMethod,
    ) {
        let key = Self::registry_key(class_name, method_name, method_descriptor);
        Self::global().write().unwrap().data.insert(key, method);
    }

    /// Register inner native methods when startup
    pub(crate) fn _register(
        data: &mut HashMap<String, NativeMethod>,
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
        method: NativeMethod,
    ) {
        let key = Self::registry_key(class_name, method_name, method_descriptor);
        data.insert(key, method);
    }

    pub fn find_native_method(
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
    ) -> Option<NativeMethod> {
        // Here we hack Object.registerNatives method, returns empty native method!
        if method_name == "registerNatives" && method_descriptor == "()V" {
            return Some(EMPTY_NATIVE_METHOD);
        }

        let key = Self::registry_key(class_name, method_name, method_descriptor);
        let registry = Self::global().read().unwrap();
        if let Some(method) = registry.data.get(&key) {
            return Some(*method);
        }

        None
    }

    /// The native method registered key formatting
    fn registry_key(class_name: &str, method_name: &str, method_descriptor: &str) -> String {
        format!("{}~{}~{}", class_name, method_name, method_descriptor)
    }
}
