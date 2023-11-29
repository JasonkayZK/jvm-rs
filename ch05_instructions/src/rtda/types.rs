use crate::rtda::object::Object;
use std::ptr::NonNull;

pub type ObjectRef = Option<NonNull<Object>>;
