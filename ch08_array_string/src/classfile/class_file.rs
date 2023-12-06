use std::cell::RefCell;
use std::rc::Rc;

use log::info;

use crate::classfile::attribute_info::{read_attributes, AttributeInfo};
use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_pool::{read_constant_pool, ConstantPool};
use crate::classfile::errors::{ClassFileError, ClassFileResult};
use crate::classfile::member_info::MemberInfo;

/// ClassFile {
///     u4             magic;
///     u2             minor_version;
///     u2             major_version;
///     u2             constant_pool_count;
///     cp_info        constant_pool[constant_pool_count-1];
///     u2             access_flags;
///     u2             this_class;
///     u2             super_class;
///     u2             interfaces_count;
///     u2             interfaces[interfaces_count];
///     u2             fields_count;
///     field_info     fields[fields_count];
///     u2             methods_count;
///     method_info    methods[methods_count];
///     u2             attributes_count;
///     attribute_info attributes[attributes_count];
/// }

const MAGIC_NUMBER: u32 = 0xCAFEBABE;

pub struct ClassFile {
    /// magic: u32,
    /// 次版本号
    minor_version: u16,
    /// 主版本号
    major_version: u16,
    /// 常量池
    constant_pool: Rc<RefCell<ConstantPool>>,
    /// 访问标志
    access_flags: u16,
    /// 类索引
    this_class: u16,
    /// 超类索引
    super_class: u16,
    /// 接口索引表
    interfaces: Vec<u16>,
    /// 类成员属性列表
    fields: Vec<MemberInfo>,
    /// 类方法列表
    methods: Vec<MemberInfo>,
    /// 其他属性
    attributes: Vec<Box<dyn AttributeInfo>>,
}

impl ClassFile {
    pub fn parse(class_data: Vec<u8>) -> ClassFileResult<ClassFile> {
        let mut class_reader = ClassReader::new(class_data);
        let mut class_file = ClassFile {
            minor_version: 0_u16,
            major_version: 0_u16,
            constant_pool: Rc::new(RefCell::new(ConstantPool::default())),
            access_flags: 0_u16,
            this_class: 0_u16,
            super_class: 0_u16,
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![],
        };
        class_file.read(&mut class_reader)?;
        Ok(class_file)
    }

    pub fn minor_version(&self) -> u16 {
        self.minor_version
    }

    pub fn major_version(&self) -> u16 {
        self.major_version
    }

    pub fn constant_pool(&self) -> &Rc<RefCell<ConstantPool>> {
        &self.constant_pool
    }

    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }

    pub fn fields(&self) -> &Vec<MemberInfo> {
        &self.fields
    }

    pub fn methods(&self) -> &Vec<MemberInfo> {
        &self.methods
    }

    pub fn attributes(&self) -> &Vec<Box<dyn AttributeInfo>> {
        &self.attributes
    }

    pub fn class_name(&self) -> String {
        self.constant_pool.borrow().get_class_name(self.this_class)
    }

    pub fn super_class_name(&self) -> String {
        if self.super_class > 0 {
            return self.constant_pool.borrow().get_class_name(self.super_class);
        }
        "".to_string()
    }

    pub fn interface_names(&self) -> Vec<String> {
        let mut interface_names = vec![];
        for i in self.interfaces.iter() {
            interface_names.push(self.constant_pool.borrow().get_class_name(*i))
        }
        interface_names.to_vec()
    }

    fn read(&mut self, reader: &mut ClassReader) -> ClassFileResult<()> {
        self.read_and_check_magic(reader)?;
        self.read_and_check_version(reader)?;

        self.constant_pool = read_constant_pool(reader);
        self.access_flags = reader.read_u16();
        self.this_class = reader.read_u16();
        self.super_class = reader.read_u16();
        self.interfaces = reader.read_u16s();
        self.fields = MemberInfo::read(reader, self.constant_pool.clone());
        self.methods = MemberInfo::read(reader, self.constant_pool.clone());
        self.attributes = read_attributes(reader, self.constant_pool.clone());

        Ok(())
    }

    fn read_and_check_magic(&mut self, reader: &mut ClassReader) -> ClassFileResult<()> {
        let magic = reader.read_u32();
        if magic != MAGIC_NUMBER {
            return Err(ClassFileError::WrongMagicNumber(magic));
        }
        Ok(())
    }

    fn read_and_check_version(&mut self, reader: &mut ClassReader) -> ClassFileResult<()> {
        self.minor_version = reader.read_u16();
        self.major_version = reader.read_u16();
        match self.major_version {
            45 => Ok(()),
            46..=52 => {
                if self.minor_version == 0 {
                    Ok(())
                } else {
                    Err(ClassFileError::UnsupportedClassVersionError(
                        self.major_version,
                    ))
                }
            }
            _ => Err(ClassFileError::UnsupportedClassVersionError(
                self.major_version,
            )),
        }
    }
}

pub fn print_class_info(class_file: &ClassFile) {
    info!(
        "version: {}.{}",
        class_file.major_version(),
        class_file.minor_version()
    );
    info!(
        "constants count: {}",
        class_file.constant_pool().borrow().infos.len()
    );
    info!("access flags: 0x{:x}", class_file.access_flags());
    info!("this class: {}", class_file.class_name());
    info!("super class: {}", class_file.super_class_name());
    info!("interfaces: {:?}", class_file.interface_names());
    info!("fields count: {:?}", class_file.fields().len());
    for field in class_file.fields() {
        info!(" {}", field.name());
    }
    info!("methods count: {:?}", class_file.methods().len());
    for method in class_file.methods() {
        info!(" {}", method.name());
    }
    info!("attributes count: {:?}", class_file.attributes().len());
    for attribute in class_file.attributes() {
        info!(" {}", attribute);
    }
}
