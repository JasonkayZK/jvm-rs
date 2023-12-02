use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use log::info;

use crate::classfile::class_file::ClassFile;
use crate::classpath::classpath_impl::ClasspathImpl;
use crate::classpath::entry::Entry;
use crate::rtda::heap::class::Class;
use crate::rtda::heap::errors::RuntimeHeapError;
use crate::rtda::heap::field::Field;
use crate::rtda::heap::object_ref::HeapObjectRefs;
use crate::types::RcRefCell;

/// class names:
///     - primitive types: boolean, byte, int ...
///     - primitive arrays: [Z, [B, [I ...
///     - non-array classes: java/lang/Object ...
///     - array classes: [Ljava/lang/Object; ...
pub struct ClassLoader {
    classpath: ClasspathImpl,
    // 保存加载的类，key 为类的完全限定名
    class_map: HashMap<String, RcRefCell<Class>>,
}

impl ClassLoader {
    pub fn new(classpath: ClasspathImpl) -> Self {
        ClassLoader {
            classpath,
            class_map: HashMap::new(),
        }
    }

    /// Load all type of classes:
    ///     - primitive types: boolean, byte, int ...
    ///     - primitive arrays: [Z, [B, [I ...
    ///     - non-array classes: java/lang/Object ...
    ///     - array classes: [Ljava/lang/Object; ...
    pub fn load_class(
        &mut self,
        class_loader_ref: RcRefCell<Self>,
        name: String,
    ) -> RcRefCell<Class> {
        match self.class_map.get(&name) {
            Some(class) => {
                // Already loaded
                class.clone()
            }
            None => self.load_non_array_class(&class_loader_ref, name),
        }
    }

    /// Load a class that is not a array
    fn load_non_array_class(
        &mut self,
        class_loader_ref: &RcRefCell<Self>,
        name: String,
    ) -> RcRefCell<Class> {
        let data = self.read_class(name.clone());
        let class = self.define_class(class_loader_ref, data);
        link(&class);
        info!("[Loaded {}", name);
        class
    }

    /// Load the classfile with given classname
    fn read_class(&mut self, name: String) -> Vec<u8> {
        match self.classpath.read_class(&name) {
            Ok(data) => data,
            Err(_) => {
                panic!("{}", RuntimeHeapError::ClassNotFound(name))
            }
        }
    }

    /// jvms 5.3.5
    /// Parse the classfile data recursively(super class load first!)
    fn define_class(
        &mut self,
        class_loader_ref: &RcRefCell<Self>,
        data: Vec<u8>,
    ) -> RcRefCell<Class> {
        let class = ClassLoader::parse_class(data);

        class
            .borrow_mut()
            .set_loader(Some(class_loader_ref.clone()));

        self.resolve_super_class(class_loader_ref, &class);
        self.resolve_interfaces(class_loader_ref, &class);

        self.class_map
            .insert(class.borrow().name().to_string(), class.clone());
        class
    }

    fn parse_class(data: Vec<u8>) -> RcRefCell<Class> {
        let cf_result = ClassFile::parse(data);
        match cf_result {
            Ok(cf) => Class::new(&cf),
            Err(err) => {
                panic!("{}", RuntimeHeapError::ParseClassFailed(err.to_string()));
            }
        }
    }

    /// jvms 5.4.3.1
    fn resolve_super_class(
        &mut self,
        class_loader_ref: &RcRefCell<Self>,
        class: &RcRefCell<Class>,
    ) {
        // java/lang/Object has no super class!
        if class.borrow_mut().name() != "java/lang/Object" {
            let super_class = Some(self.load_class(
                class_loader_ref.clone(),
                class.borrow_mut().super_classname().to_string(),
            ));
            class.borrow_mut().set_super_class(super_class);
        }
    }

    fn resolve_interfaces(&mut self, class_loader_ref: &RcRefCell<Self>, class: &RcRefCell<Class>) {
        let mut class = class.borrow_mut();
        let interface_names = class.interface_names();
        let mut interfaces: Vec<RcRefCell<Class>> = Vec::new();
        for name in interface_names {
            interfaces.push(self.load_class(class_loader_ref.clone(), name.to_string()));
        }
        class.set_interfaces(Some(interfaces));
    }
}

fn link(class: &RcRefCell<Class>) {
    verify(class);
    prepare(class);
}

fn verify(_class: &RcRefCell<Class>) {
    // TODO
}

/// jvms 5.4.2
fn prepare(class: &RcRefCell<Class>) {
    calc_instance_field_object_ref_ids(class);
    calc_static_field_object_ref_ids(class);
    alloc_and_init_static_vars(class);
}

fn calc_instance_field_object_ref_ids(class: &RcRefCell<Class>) {
    let mut object_ref_id = 0;
    if class.borrow_mut().super_class().is_some() {
        object_ref_id = class
            .borrow_mut()
            .super_class()
            .unwrap()
            .borrow()
            .instance_object_ref_count();
    }
    for field in class.borrow_mut().fields() {
        if !field.borrow_mut().is_static() {
            field.borrow_mut().set_object_ref_id(object_ref_id);
            object_ref_id += 1;
            if field.borrow_mut().is_long_or_double() {
                object_ref_id += 1;
            }
        }
    }
    class
        .borrow_mut()
        .set_instance_object_ref_count(object_ref_id);
}

fn calc_static_field_object_ref_ids(class: &RcRefCell<Class>) {
    let mut object_ref_id = 0;
    for field in class.borrow_mut().fields() {
        if field.borrow_mut().is_static() {
            field.borrow_mut().set_object_ref_id(object_ref_id);
            object_ref_id += 1;
            if field.borrow_mut().is_long_or_double() {
                object_ref_id += 1;
            }
        }
    }
    class
        .borrow_mut()
        .set_static_object_ref_count(object_ref_id);
}

fn alloc_and_init_static_vars(class: &RcRefCell<Class>) {
    let static_object_ref_count = class.borrow_mut().static_object_ref_count() as usize;
    let vars = Some(Rc::new(RefCell::new(HeapObjectRefs::new(
        static_object_ref_count,
    ))));
    let fields = class.borrow_mut().fields();
    for field in fields {
        if field.borrow().is_static() && field.borrow().is_final() {
            init_static_final_var(class, vars.as_ref().unwrap(), &field);
        }
    }
    class.borrow_mut().set_static_vars(vars);
}

fn init_static_final_var(
    class: &RcRefCell<Class>,
    vars: &RcRefCell<HeapObjectRefs>,
    field: &RcRefCell<Field>,
) {
    let cp = class.borrow_mut().constant_pool();
    let field = field.borrow();
    let cp_index = field.const_value_index();
    let object_ref_id = field.object_ref_id();
    if cp_index > 0 {
        let descriptor = field.descriptor();
        if descriptor == "Z"
            || descriptor == "B"
            || descriptor == "C"
            || descriptor == "S"
            || descriptor == "I"
        {
            let val = *cp
                .borrow()
                .get_constant(cp_index as usize)
                .as_any()
                .downcast_ref::<i32>()
                .unwrap();
            vars.borrow_mut().set_int(object_ref_id as usize, val);
        } else if descriptor == "J" {
            let val = *cp
                .borrow()
                .get_constant(cp_index as usize)
                .as_any()
                .downcast_ref::<i64>()
                .unwrap();
            vars.borrow_mut().set_long(object_ref_id as usize, val);
        } else if descriptor == "F" {
            let val = *cp
                .borrow()
                .get_constant(cp_index as usize)
                .as_any()
                .downcast_ref::<f32>()
                .unwrap();
            vars.borrow_mut().set_float(object_ref_id as usize, val);
        } else if descriptor == "D" {
            let val = *cp
                .borrow()
                .get_constant(cp_index as usize)
                .as_any()
                .downcast_ref::<f64>()
                .unwrap();
            vars.borrow_mut().set_double(object_ref_id as usize, val);
        } else if descriptor == "Ljava/lang/String;" {
            panic!("Todo");
        }
    }
}
