use std::cell::RefCell;
use std::rc::Rc;

use crate::rtda::object::Object;

pub type ObjectRef = Option<Rc<RefCell<Object>>>;
