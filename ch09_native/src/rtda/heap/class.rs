use std::cell::RefCell;
use std::rc::Rc;

use crate::classfile::class_file::ClassFile;
use crate::rtda::heap::access_flags::{
    ACC_ABSTRACT, ACC_ANNOTATION, ACC_ENUM, ACC_FINAL, ACC_INTERFACE, ACC_PUBLIC, ACC_SUPER,
    ACC_SYNTHETIC,
};
use crate::rtda::heap::class_loader::ClassLoader;
use crate::rtda::heap::class_name_helper::get_array_class_name;
use crate::rtda::heap::consts::{CLONEABLE_CLASS, OBJECT_CLASS, SERIALIZABLE_CLASS};
use crate::rtda::heap::field::Field;
use crate::rtda::heap::method::Method;
use crate::rtda::heap::object_ref::HeapObjectRefs;
use crate::rtda::heap::runtime_constant_pool::RuntimeConstantPool;
use crate::rtda::object::Object;
use crate::types::{OptionRcRefCell, OptionVecRcRefCell, RcRefCell, VecRcRefCell};

pub struct Class {
    access_flags: u16,
    name: String,
    super_classname: String,
    interface_names: Vec<String>,
    constant_pool: OptionRcRefCell<RuntimeConstantPool>,
    fields: OptionVecRcRefCell<Field>,
    methods: OptionVecRcRefCell<Method>,
    loader: OptionRcRefCell<ClassLoader>,
    super_class: OptionRcRefCell<Class>,
    interfaces: OptionVecRcRefCell<Class>,
    instance_object_ref_count: u64,
    static_object_ref_count: u64,
    static_vars: OptionRcRefCell<HeapObjectRefs>,

    init_started: bool,
}

impl Class {
    pub fn new(cf: &ClassFile) -> RcRefCell<Self> {
        let class = Class {
            access_flags: cf.access_flags(),
            name: cf.class_name(),
            super_classname: cf.super_class_name(),
            interface_names: cf.interface_names(),
            constant_pool: None,
            fields: None,
            methods: None,
            loader: None,
            super_class: None,
            interfaces: None,
            instance_object_ref_count: 0,
            static_object_ref_count: 0,
            static_vars: None,
            init_started: false,
        };
        let rc_class = Rc::new(RefCell::new(class));
        rc_class.borrow_mut().constant_pool = Some(RuntimeConstantPool::new(
            rc_class.clone(),
            cf.constant_pool(),
        ));
        rc_class.borrow_mut().fields = Some(Field::new_fields(rc_class.clone(), cf.fields()));
        rc_class.borrow_mut().methods = Some(Method::new_methods(rc_class.clone(), cf.methods()));
        rc_class
    }

    pub fn new_array_class(name: String) -> RcRefCell<Self> {
        let class = Class {
            access_flags: ACC_PUBLIC,
            name,
            super_classname: OBJECT_CLASS.into(),
            interface_names: vec![CLONEABLE_CLASS.into(), SERIALIZABLE_CLASS.into()],
            constant_pool: None,
            fields: Some(vec![]),
            methods: Some(vec![]),
            loader: None,
            super_class: None,
            interfaces: None,
            instance_object_ref_count: 0,
            static_object_ref_count: 0,
            static_vars: None,
            init_started: true,
        };
        Rc::new(RefCell::new(class))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> VecRcRefCell<Field> {
        self.fields.clone().unwrap()
    }

    pub fn methods(&self) -> VecRcRefCell<Method> {
        self.methods.clone().unwrap()
    }

    pub fn super_classname(&self) -> &str {
        &self.super_classname
    }

    pub fn interface_names(&self) -> &Vec<String> {
        &self.interface_names
    }

    pub fn set_loader(&mut self, loader: OptionRcRefCell<ClassLoader>) {
        self.loader = loader;
    }

    pub fn loader(&self) -> OptionRcRefCell<ClassLoader> {
        self.loader.clone()
    }

    pub fn constant_pool(&self) -> RcRefCell<RuntimeConstantPool> {
        self.constant_pool.clone().unwrap()
    }

    pub fn set_super_class(&mut self, super_class: OptionRcRefCell<Class>) {
        self.super_class = super_class;
    }

    pub fn super_class(&self) -> OptionRcRefCell<Class> {
        self.super_class.clone()
    }

    pub fn set_interfaces(&mut self, interfaces: OptionVecRcRefCell<Class>) {
        self.interfaces = interfaces;
    }

    pub fn interfaces(&self) -> OptionVecRcRefCell<Class> {
        self.interfaces.clone()
    }

    pub fn set_instance_object_ref_count(&mut self, count: u64) {
        self.instance_object_ref_count = count;
    }

    pub fn instance_object_ref_count(&self) -> u64 {
        self.instance_object_ref_count
    }

    pub fn set_static_object_ref_count(&mut self, count: u64) {
        self.static_object_ref_count = count;
    }

    pub fn static_object_ref_count(&self) -> u64 {
        self.static_object_ref_count
    }

    pub fn set_static_vars(&mut self, static_vars: OptionRcRefCell<HeapObjectRefs>) {
        self.static_vars = static_vars;
    }

    pub fn static_vars(&self) -> RcRefCell<HeapObjectRefs> {
        self.static_vars.clone().unwrap()
    }

    pub fn is_public(&self) -> bool {
        self.access_flags & ACC_PUBLIC != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags & ACC_FINAL != 0
    }

    pub fn is_super(&self) -> bool {
        self.access_flags & ACC_SUPER != 0
    }

    pub fn is_interface(&self) -> bool {
        self.access_flags & ACC_INTERFACE != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.access_flags & ACC_ABSTRACT != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.access_flags & ACC_SYNTHETIC != 0
    }

    pub fn is_annotation(&self) -> bool {
        self.access_flags & ACC_ANNOTATION != 0
    }

    pub fn is_enum(&self) -> bool {
        self.access_flags & ACC_ENUM != 0
    }

    pub fn get_main_method(&self) -> OptionRcRefCell<Method> {
        self.get_static_method("main".into(), "([Ljava/lang/String;)V".into())
    }

    pub fn get_clinit_method(&self) -> OptionRcRefCell<Method> {
        self.get_static_method("<clinit>".into(), "()V".into())
    }

    pub fn get_package_name(&self) -> String {
        match self.name.rfind('/') {
            Some(i) => self.name.as_str()[..i].into(),
            None => "".into(),
        }
    }

    fn get_static_method(&self, name: String, descriptor: String) -> OptionRcRefCell<Method> {
        for method in self.methods.as_ref().unwrap() {
            let b_method = method.borrow();
            if b_method.is_static()
                && b_method.name() == name
                && b_method.descriptor() == descriptor
            {
                return Some(method.clone());
            }
        }
        None
    }

    /// jvms 5.4.4
    ///
    /// Check object could be accessed by other class
    pub fn is_accessible_to(&self, other: &Class) -> bool {
        self.is_public() || self.get_package_name() == other.get_package_name()
    }

    /// jvms8 6.5.instanceof
    /// jvms8 6.5.checkcast
    pub fn is_assignable_from(
        &self,
        self_ref: &RcRefCell<Class>,
        other: &RcRefCell<Class>,
    ) -> bool {
        if self_ref.eq(other) {
            return true;
        }

        return if !other.borrow().is_array() {
            if !other.borrow().is_interface() {
                // other is class
                if !self_ref.borrow().is_interface() {
                    // self_ref is not interface
                    other.borrow().is_sub_class_of(self_ref)
                } else {
                    // self_ref is interface
                    other.borrow().is_implements(self_ref)
                }
            } else {
                // other is interface
                if !self_ref.borrow().is_interface() {
                    // self_ref is not interface
                    self_ref.borrow().is_jl_object()
                } else {
                    // self_ref is interface
                    other.borrow().is_sub_interface_of(self_ref)
                }
            }
        } else {
            // other is array
            if !self_ref.borrow().is_array() {
                if !self_ref.borrow().is_interface() {
                    // self_ref is class
                    self_ref.borrow().is_jl_object()
                } else {
                    // self_ref is interface
                    self_ref.borrow().is_jl_cloneable() || self_ref.borrow().is_jio_serializable()
                }
            } else {
                // self_ref is array
                let oc = other.borrow_mut().component_class();
                let sc = self_ref.borrow_mut().component_class();
                oc.eq(&sc) || sc.borrow().is_assignable_from(&sc, &oc)
            }
        };
    }

    /// self extends other
    pub fn is_sub_class_of(&self, other: &RcRefCell<Class>) -> bool {
        let mut c = self.super_class();
        while let Some(class) = c {
            if class.eq(other) {
                return true;
            }
            c = class.borrow().super_class();
        }
        false
    }

    /// self implements other
    pub fn is_implements(&self, other: &RcRefCell<Class>) -> bool {
        for i in self.interfaces().unwrap() {
            if i.eq(other) || i.borrow().is_sub_interface_of(other) {
                return true;
            }
        }

        let mut c = self.super_class();
        while let Some(class) = c {
            for i in class.borrow().interfaces().unwrap() {
                if i.eq(other) || i.borrow().is_sub_interface_of(other) {
                    return true;
                }
            }

            c = class.borrow().super_class();
        }
        false
    }

    /// self extends other
    pub fn is_sub_interface_of(&self, other: &RcRefCell<Class>) -> bool {
        for super_interface in self.interfaces().unwrap() {
            if super_interface.eq(other) || super_interface.borrow().is_sub_interface_of(other) {
                return true;
            }
        }
        false
    }

    pub fn start_init(&mut self) {
        self.init_started = true;
    }

    pub fn init_started(&self) -> bool {
        self.init_started
    }

    pub fn new_object(&self, class: RcRefCell<Class>) -> Object {
        Object::new(class)
    }

    pub fn array_class(&mut self) -> RcRefCell<Class> {
        let array_class_name = get_array_class_name(&self.name);
        let loader = self.loader.as_mut().unwrap();
        return loader
            .borrow_mut()
            .load_class(loader.clone(), array_class_name);
    }

    pub fn get_field(
        &self,
        name: String,
        descriptor: String,
        is_static: bool,
    ) -> OptionRcRefCell<Field> {
        for field in self.fields.as_ref().unwrap() {
            if field.borrow().is_static() == is_static
                && field.borrow().name() == name
                && field.borrow().descriptor() == descriptor
            {
                return Some(field.clone());
            }
        }

        let mut c = self.super_class();
        while let Some(class) = c {
            for field in class.borrow().fields() {
                if field.borrow().is_static() == is_static
                    && field.borrow().name() == name
                    && field.borrow().descriptor() == descriptor
                {
                    return Some(field);
                }
            }

            c = class.borrow().super_class();
        }
        None
    }

    fn is_jl_object(&self) -> bool {
        self.name == OBJECT_CLASS
    }

    fn is_jl_cloneable(&self) -> bool {
        self.name == CLONEABLE_CLASS
    }

    fn is_jio_serializable(&self) -> bool {
        self.name == SERIALIZABLE_CLASS
    }
}

impl PartialEq<Self> for Class {
    fn eq(&self, other: &Self) -> bool {
        let self_ptr = self as *const Self;
        let other_ptr = other as *const Self;
        self_ptr.eq(&other_ptr)
    }
}
