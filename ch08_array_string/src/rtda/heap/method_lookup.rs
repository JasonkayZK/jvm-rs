use crate::rtda::heap::class::Class;
use crate::rtda::heap::method::Method;
use crate::types::{OptionRcRefCell, RcRefCell};

pub fn lookup_method_in_class(
    class: &RcRefCell<Class>,
    name: String,
    descriptor: String,
) -> OptionRcRefCell<Method> {
    let mut c = Some(class.clone());
    while let Some(class) = c {
        for method in class.borrow().methods() {
            if method.borrow().name() == name && method.borrow().descriptor() == descriptor {
                return Some(method);
            }
        }
        c = class.borrow().super_class();
    }
    None
}

pub fn lookup_method_in_interfaces(
    ifaces: &Vec<RcRefCell<Class>>,
    name: String,
    descriptor: String,
) -> OptionRcRefCell<Method> {
    for iface in ifaces {
        for method in iface.borrow().methods() {
            if method.borrow().name() == name && method.borrow().descriptor() == descriptor {
                return Some(method);
            }
        }
        let method = lookup_method_in_interfaces(
            iface.borrow().interfaces().as_ref().unwrap(),
            name.clone(),
            descriptor.clone(),
        );
        if method.is_some() {
            return method;
        }
    }
    None
}
