use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex, OnceLock};

use crate::rtda::heap::class_loader::ClassLoader;
use crate::rtda::heap::consts::STRING_CLASS;
use crate::rtda::object::Object;
use crate::types::{ArcMutex, RcRefCell};

/// StringPool stores the pointer points to the java string object
///
/// ArcMutex is used to ensure access in concurrent condition.

struct JStringObjectPtr {
    pub data: RcRefCell<Object>,
}

unsafe impl Send for JStringObjectPtr {}

pub struct StringPool {
    pool: HashMap<String, JStringObjectPtr>,
}

impl StringPool {
    pub fn global() -> &'static ArcMutex<StringPool> {
        static STRING_POOL: OnceLock<ArcMutex<StringPool>> = OnceLock::new();
        STRING_POOL.get_or_init(|| {
            Arc::new(Mutex::new(StringPool {
                pool: HashMap::new(),
            }))
        })
    }

    /// Convert the rust string into a Java string object
    ///
    /// The java object will be stored in current StringPool
    pub fn jstring(&mut self, loader: RcRefCell<ClassLoader>, rstr: String) -> RcRefCell<Object> {
        if let Some(interned_str) = self.pool.get(rstr.as_str()) {
            return interned_str.data.clone();
        }

        let loader_mut = unsafe { &mut *loader.as_ptr() };

        let chars = string_to_utf16(rstr.clone());
        let j_chars = Object::new_data(
            loader_mut.load_class(loader.clone(), "[C".into()),
            Box::new(chars),
        );

        let class = loader_mut.load_class(loader, STRING_CLASS.into());
        let mut j_str = Rc::new(RefCell::new(class.borrow().new_object(class.clone())));
        j_str
            .borrow_mut()
            .set_ref_var("value".into(), "[C".into(), Rc::new(RefCell::new(j_chars)));

        self.pool.insert(
            rstr,
            JStringObjectPtr {
                data: j_str.clone(),
            },
        );

        j_str
    }

    pub fn is_exist(&self, rstr: String) -> bool {
        let interned_str = self.pool.get(rstr.as_str());
        if interned_str.is_some() {
            return true;
        }
        false
    }

    pub fn add(&mut self, rstr: String, jstring: RcRefCell<Object>) {
        self.pool.insert(rstr, JStringObjectPtr { data: jstring });
    }
}

/// java.lang.String -> rust String
pub fn rust_string(obj: &RcRefCell<Object>) -> String {
    let char_arr = obj.borrow_mut().get_ref_var("value".into(), "[C".into());
    let chars = char_arr.borrow_mut().chars_mut().clone();
    utf16_to_string(chars)
}

/// utf-8 -> utf16
pub fn string_to_utf16(s: String) -> Vec<u16> {
    s.encode_utf16().collect::<Vec<u16>>()
}

/// utf16 -> utf-8
pub fn utf16_to_string(s: Vec<u16>) -> String {
    String::from_utf16(&s).unwrap()
}

/// java.lang.String -> rust String
pub fn intern_string(obj: &RcRefCell<Object>) -> RcRefCell<Object> {
    let rust_string = rust_string(obj);
    let class = obj.borrow().class().clone();
    let loader = class.borrow().loader().unwrap();
    let mut string_pool = StringPool::global().lock().unwrap();
    if string_pool.is_exist(rust_string.clone()) {
        return string_pool.jstring(loader, rust_string);
    }
    string_pool.add(rust_string, obj.clone());

    obj.clone()
}
