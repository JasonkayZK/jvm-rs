use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::rtda::heap::consts::ArrayTypeEnum;
use crate::rtda::heap::errors::RuntimeHeapError;
use crate::rtda::heap::object_ref::HeapObjectRefs;
use crate::rtda::object::{Object, ObjectData};
use crate::types::{ObjectRef, OptionRcRefCell, RcRefCell};

impl Object {
    pub fn clone_object(&self, obj: &RcRefCell<Object>) -> ObjectRef {
        let obj = obj.borrow();

        Some(Rc::new(RefCell::new(Object::new_data(
            obj.class().clone(),
            Object::clone_data(obj),
        ))))
    }

    fn clone_data(obj: Ref<Object>) -> Box<dyn ObjectData> {
        let old_data = obj.data();
        let tag = old_data.tag();

        if let Ok(array_type) = ArrayTypeEnum::try_from(tag) {
            // The object is an array
            match array_type {
                ArrayTypeEnum::Bytes => {
                    let old_refs = old_data.as_any().downcast_ref::<Vec<i8>>().unwrap();
                    Box::new(old_refs.clone()) as Box<dyn ObjectData>
                }
                ArrayTypeEnum::Shorts => {
                    let old_refs = old_data.as_any().downcast_ref::<Vec<i16>>().unwrap();
                    Box::new(old_refs.clone()) as Box<dyn ObjectData>
                }
                ArrayTypeEnum::Ints => {
                    let old_refs = old_data.as_any().downcast_ref::<Vec<i32>>().unwrap();
                    Box::new(old_refs.clone()) as Box<dyn ObjectData>
                }
                ArrayTypeEnum::Longs => {
                    let old_refs = old_data.as_any().downcast_ref::<Vec<i64>>().unwrap();
                    Box::new(old_refs.clone()) as Box<dyn ObjectData>
                }
                ArrayTypeEnum::Chars => {
                    let old_refs = old_data.as_any().downcast_ref::<Vec<u16>>().unwrap();
                    Box::new(old_refs.clone()) as Box<dyn ObjectData>
                }
                ArrayTypeEnum::Floats => {
                    let old_refs = old_data.as_any().downcast_ref::<Vec<f32>>().unwrap();
                    Box::new(old_refs.clone()) as Box<dyn ObjectData>
                }
                ArrayTypeEnum::Doubles => {
                    let old_refs = old_data.as_any().downcast_ref::<Vec<f64>>().unwrap();
                    Box::new(old_refs.clone()) as Box<dyn ObjectData>
                }
                ArrayTypeEnum::Refs => {
                    let old_refs = old_data
                        .as_any()
                        .downcast_ref::<Vec<OptionRcRefCell<Object>>>()
                        .unwrap();
                    Box::new(old_refs.clone()) as Box<dyn ObjectData>
                }
                ArrayTypeEnum::Unknown => {
                    panic!("{}", RuntimeHeapError::UnknownArrayType(tag))
                }
            }
        } else {
            // The object is an pure object
            let old_refs = old_data.as_any().downcast_ref::<HeapObjectRefs>().unwrap();
            Box::new(old_refs.clone()) as Box<dyn ObjectData>
        }
    }
}
