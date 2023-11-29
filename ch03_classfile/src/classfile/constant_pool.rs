use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_pool::class::ConstantClassInfo;
use crate::classfile::constant_pool::constant_info::ConstantInfo;
use crate::classfile::constant_pool::invoke_dynamic::{
    ConstantInvokeDynamicInfo, ConstantMethodHandleInfo, ConstantMethodTypeInfo,
};
use crate::classfile::constant_pool::member_ref::{
    ConstantFieldRefInfo, ConstantInterfaceMethodRefInfo, ConstantMethodRefInfo,
};
use crate::classfile::constant_pool::name_and_type::ConstantNameAndTypeInfo;
use crate::classfile::constant_pool::numeric::{
    ConstantDoubleInfo, ConstantFloatInfo, ConstantIntegerInfo, ConstantLongInfo,
};
use crate::classfile::constant_pool::string::ConstantStringInfo;
use crate::classfile::constant_pool::utf8::ConstantUtf8Info;

mod class;
mod constant_info;
mod consts;
mod invoke_dynamic;
mod member_ref;
mod name_and_type;
mod numeric;
mod string;
mod utf8;

#[derive(Default)]
pub struct ConstantPool {
    pub infos: Vec<Option<Box<dyn ConstantInfo>>>,

    /// 存储 CONSTANT_Class_info 常量映射
    class_info_map: HashMap<u16, ConstantClassInfo>,
    /// 存储 CONSTANT_Utf8_info 常量映射
    utf8_info_map: HashMap<u16, ConstantUtf8Info>,
}

impl ConstantPool {
    pub fn get_class_name(&self, index: u16) -> String {
        match self.class_info_map.get(&index) {
            Some(info) => info.name(),
            None => "".to_string(),
        }
    }

    pub fn get_utf8(&self, index: u16) -> String {
        match self.utf8_info_map.get(&index) {
            Some(info) => info.str(),
            None => "".to_string(),
        }
    }
}

pub fn read_constant_pool(reader: &mut ClassReader) -> Rc<RefCell<ConstantPool>> {
    let cp_count = reader.read_u16();
    let cp = Rc::new(RefCell::new(ConstantPool::default()));
    // 第一个元素无效
    cp.borrow_mut().infos.push(None);
    // 常量池索引从 1 到 constant_pool_count - 1.
    let mut i = 1;
    loop {
        if i == cp_count {
            break;
        }
        let constant_info = read_constant_info(reader, i, cp.clone());
        // http://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4.5
        // All 8-byte constants take up two entries in the constant_pool table of the class file.
        // If a CONSTANT_Long_info or CONSTANT_Double_info structure is the item in the constant_pool
        // table at index n, then the next usable item in the pool is located at index n+2.
        // The constant_pool index n+1 must be valid but is considered unusable.
        match (&constant_info).tag() {
            // CONSTANT_Long_info 和 CONSTANT_Double_info 各占两个位置
            consts::CONSTANT_LONG | consts::CONSTANT_DOUBLE => {
                cp.borrow_mut().infos.push(Some(constant_info));
                cp.borrow_mut().infos.push(None);
                i += 1;
            }
            _ => {
                cp.borrow_mut().infos.push(Some(constant_info));
            }
        }

        i += 1;
    }

    cp
}

fn read_constant_info(
    reader: &mut ClassReader,
    i: u16,
    cp: Rc<RefCell<ConstantPool>>,
) -> Box<dyn ConstantInfo> {
    let tag = reader.read_u8();
    let mut c = new_constant_info(reader, tag, i, cp);
    match (&c).tag() {
        // CONSTANT_Utf8_info、CONSTANT_Class_info 在创建之后立即调用 read_info
        consts::CONSTANT_UTF8 | consts::CONSTANT_CLASS => {}
        _ => c.read_info(reader),
    }
    c
}

fn new_constant_info(
    reader: &mut ClassReader,
    tag: u8,
    i: u16,
    cp: Rc<RefCell<ConstantPool>>,
) -> Box<dyn ConstantInfo> {
    match tag {
        consts::CONSTANT_CLASS => {
            let mut b = Box::new(ConstantClassInfo::new(cp.clone()));
            // 立即调用
            b.read_info(reader);
            cp.borrow_mut().class_info_map.insert(i, *b.clone());
            b
        }
        consts::CONSTANT_FIELD_REF => Box::new(ConstantFieldRefInfo::new(cp)),
        consts::CONSTANT_METHOD_REF => Box::new(ConstantMethodRefInfo::new(cp)),
        consts::CONSTANT_INTERFACE_METHOD_REF => Box::new(ConstantInterfaceMethodRefInfo::new(cp)),
        consts::CONSTANT_STRING => Box::new(ConstantStringInfo::new(cp)),
        consts::CONSTANT_INTEGER => Box::new(ConstantIntegerInfo::default()),
        consts::CONSTANT_FLOAT => Box::new(ConstantFloatInfo::default()),
        consts::CONSTANT_LONG => Box::new(ConstantLongInfo::default()),
        consts::CONSTANT_DOUBLE => Box::new(ConstantDoubleInfo::default()),
        consts::CONSTANT_NAME_AND_TYPE => Box::new(ConstantNameAndTypeInfo::default()),
        consts::CONSTANT_UTF8 => {
            let mut b = Box::new(ConstantUtf8Info::default());
            // 立即调用
            b.read_info(reader);
            cp.borrow_mut().utf8_info_map.insert(i, *b.clone());
            b
        }
        consts::CONSTANT_METHOD_HANDLE => Box::new(ConstantMethodHandleInfo::default()),
        consts::CONSTANT_METHOD_TYPE => Box::new(ConstantMethodTypeInfo::default()),
        consts::CONSTANT_INVOKE_DYNAMIC => Box::new(ConstantInvokeDynamicInfo::default()),
        _ => panic!(
            "{}",
            "java.lang.ClassFormatError: unknown constant pool tag!"
        ),
    }
}
