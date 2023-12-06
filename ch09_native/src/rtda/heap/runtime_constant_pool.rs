use std::cell::RefCell;
use std::rc::Rc;

use crate::classfile::constant_pool::class::ConstantClassInfo;
use crate::classfile::constant_pool::member_ref::{
    ConstantFieldRefInfo, ConstantInterfaceMethodRefInfo, ConstantMethodRefInfo,
};
use crate::classfile::constant_pool::numeric::{
    ConstantDoubleInfo, ConstantFloatInfo, ConstantIntegerInfo, ConstantLongInfo,
};
use crate::classfile::constant_pool::string::ConstantStringInfo;
use crate::classfile::constant_pool::{consts, ConstantPool};
use crate::rtda::heap::class::Class;
use crate::rtda::heap::class_ref::ClassRef;
use crate::rtda::heap::constant::Constant;
use crate::rtda::heap::errors::RuntimeHeapError;
use crate::rtda::heap::field_ref::FieldRef;
use crate::rtda::heap::interface_ref::InterfaceMethodRef;
use crate::rtda::heap::method_ref::MethodRef;
use crate::types::RcRefCell;

/// 运行时常量池
pub struct RuntimeConstantPool {
    class: RcRefCell<Class>,
    consts: Vec<Option<Box<dyn Constant>>>,
}

impl RuntimeConstantPool {
    /// 将常量池转化成运行时常量池
    pub fn new(class: RcRefCell<Class>, cf_cp: &RcRefCell<ConstantPool>) -> RcRefCell<Self> {
        let b_cf_cp = cf_cp.borrow();
        let len = b_cf_cp.constant_len();
        let consts: Vec<Option<Box<dyn Constant>>> = Vec::new();
        let rt_cp = Rc::new(RefCell::new(RuntimeConstantPool { class, consts }));
        rt_cp.borrow_mut().consts.push(None);

        let mut i = 1;
        loop {
            if i == len {
                break;
            }
            let cp_info = b_cf_cp.get_constant_info(i).as_ref().unwrap();
            match cp_info.tag() {
                consts::CONSTANT_INTEGER => {
                    let int_info = cp_info
                        .as_any()
                        .downcast_ref::<ConstantIntegerInfo>()
                        .unwrap();
                    rt_cp
                        .borrow_mut()
                        .consts
                        .push(Some(Box::new(int_info.value())));
                }
                consts::CONSTANT_FLOAT => {
                    let float_info = cp_info
                        .as_any()
                        .downcast_ref::<ConstantFloatInfo>()
                        .unwrap();
                    rt_cp
                        .borrow_mut()
                        .consts
                        .push(Some(Box::new(float_info.value())));
                }
                consts::CONSTANT_LONG => {
                    let long_info = cp_info.as_any().downcast_ref::<ConstantLongInfo>().unwrap();
                    // 占两个位置
                    rt_cp
                        .borrow_mut()
                        .consts
                        .push(Some(Box::new(long_info.value())));
                    rt_cp.borrow_mut().consts.push(None);
                    i += 1;
                }
                consts::CONSTANT_DOUBLE => {
                    let double_info = cp_info
                        .as_any()
                        .downcast_ref::<ConstantDoubleInfo>()
                        .unwrap();
                    // 占两个位置
                    rt_cp
                        .borrow_mut()
                        .consts
                        .push(Some(Box::new(double_info.value())));
                    rt_cp.borrow_mut().consts.push(None);
                    i += 1;
                }
                consts::CONSTANT_STRING => {
                    let string_info = cp_info
                        .as_any()
                        .downcast_ref::<ConstantStringInfo>()
                        .unwrap();
                    rt_cp
                        .borrow_mut()
                        .consts
                        .push(Some(Box::new(string_info.get_string())));
                }
                consts::CONSTANT_CLASS => {
                    let class_info = cp_info
                        .as_any()
                        .downcast_ref::<ConstantClassInfo>()
                        .unwrap();
                    rt_cp
                        .borrow_mut()
                        .consts
                        .push(Some(Box::new(ClassRef::new(class_info))));
                }
                consts::CONSTANT_FIELD_REF => {
                    let field_ref_info = cp_info
                        .as_any()
                        .downcast_ref::<ConstantFieldRefInfo>()
                        .unwrap();
                    rt_cp
                        .borrow_mut()
                        .consts
                        .push(Some(Box::new(FieldRef::new(field_ref_info))));
                }
                consts::CONSTANT_METHOD_REF => {
                    let method_ref_info = cp_info
                        .as_any()
                        .downcast_ref::<ConstantMethodRefInfo>()
                        .unwrap();
                    rt_cp
                        .borrow_mut()
                        .consts
                        .push(Some(Box::new(MethodRef::new(method_ref_info))));
                }
                consts::CONSTANT_INTERFACE_METHOD_REF => {
                    let interface_method_ref_info = cp_info
                        .as_any()
                        .downcast_ref::<ConstantInterfaceMethodRefInfo>()
                        .unwrap();
                    rt_cp
                        .borrow_mut()
                        .consts
                        .push(Some(Box::new(InterfaceMethodRef::new(
                            interface_method_ref_info,
                        ))));
                }
                _ => {
                    rt_cp.borrow_mut().consts.push(None);
                }
            }
            i += 1
        }
        rt_cp
    }

    pub fn class(&self) -> RcRefCell<Class> {
        self.class.clone()
    }

    pub fn get_constant(&self, index: usize) -> &dyn Constant {
        match self.consts.get(index) {
            Some(_const) => _const.as_deref().unwrap(),
            None => {
                panic!("{}", RuntimeHeapError::ConstantNotFound(index));
            }
        }
    }

    pub fn get_constant_mut(&mut self, index: usize) -> &mut Box<dyn Constant> {
        match self.consts.get_mut(index) {
            Some(_const) => _const.as_mut().unwrap(),
            None => {
                panic!("{}", RuntimeHeapError::ConstantNotFound(index));
            }
        }
    }
}
