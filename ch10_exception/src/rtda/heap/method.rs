use std::cell::RefCell;
use std::rc::Rc;

use crate::classfile::attribute_info::line_number::LineNumberTableAttribute;
use crate::classfile::member_info::MemberInfo;
use crate::rtda::heap::access_flags::{
    ACC_ABSTRACT, ACC_BRIDGE, ACC_FINAL, ACC_NATIVE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC,
    ACC_STATIC, ACC_STRICT, ACC_SYNCHRONIZED, ACC_SYNTHETIC, ACC_VARARGS,
};
use crate::rtda::heap::class::Class;
use crate::rtda::heap::exception_table::ExceptionTable;
use crate::rtda::heap::method_descriptor::MethodDescriptorParser;
use crate::types::{OptionRcRefCell, RcRefCell};

#[derive(Default)]
pub struct Method {
    access_flags: u16,
    name: String,
    descriptor: String,
    class: OptionRcRefCell<Class>,

    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,

    arg_object_ref_count: u64,

    // Exception implementation
    exception_table: Option<ExceptionTable>,
    line_number_table: Option<LineNumberTableAttribute>,
}

impl Method {
    pub fn new_methods(
        class: RcRefCell<Class>,
        cf_methods: &Vec<MemberInfo>,
    ) -> Vec<RcRefCell<Method>> {
        let mut methods = Vec::new();
        for m in cf_methods {
            methods.push(Method::new(class.clone(), m));
        }
        methods
    }

    pub fn calc_arg_object_ref_count(&mut self, parsed_descriptor: &Vec<String>) {
        for param_type in parsed_descriptor {
            self.arg_object_ref_count += 1;
            if param_type == "J" || param_type == "D" {
                self.arg_object_ref_count += 1;
            }
        }
        if !self.is_static() {
            self.arg_object_ref_count += 1; // `this` reference
        }
    }

    pub fn is_public(&self) -> bool {
        self.access_flags & ACC_PUBLIC != 0
    }

    pub fn is_private(&self) -> bool {
        self.access_flags & ACC_PRIVATE != 0
    }

    pub fn is_protected(&self) -> bool {
        self.access_flags & ACC_PROTECTED != 0
    }

    pub fn is_static(&self) -> bool {
        self.access_flags & ACC_STATIC != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags & ACC_FINAL != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.access_flags & ACC_SYNTHETIC != 0
    }

    pub fn is_synchronized(&self) -> bool {
        self.access_flags & ACC_SYNCHRONIZED != 0
    }

    pub fn is_bridge(&self) -> bool {
        self.access_flags & ACC_BRIDGE != 0
    }

    pub fn is_varargs(&self) -> bool {
        self.access_flags & ACC_VARARGS != 0
    }

    pub fn is_native(&self) -> bool {
        self.access_flags & ACC_NATIVE != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.access_flags & ACC_ABSTRACT != 0
    }

    pub fn is_strict(&self) -> bool {
        self.access_flags & ACC_STRICT != 0
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn descriptor(&self) -> String {
        self.descriptor.clone()
    }

    pub fn get_class(&self) -> RcRefCell<Class> {
        self.class.clone().unwrap()
    }

    pub fn max_stack(&self) -> u16 {
        self.max_stack
    }

    pub fn max_locals(&self) -> u16 {
        self.max_locals
    }

    pub fn code(&self) -> Vec<u8> {
        self.code.clone()
    }

    pub fn arg_object_ref_count(&self) -> u64 {
        self.arg_object_ref_count
    }

    /// jvms 5.4.4
    pub fn is_accessible_to(&self, class: &RcRefCell<Class>) -> bool {
        if self.is_public() {
            return true;
        }
        let c = self.class.as_ref().unwrap();
        if self.is_protected() {
            return class.eq(c)
                || class.borrow().is_sub_class_of(c)
                || c.borrow().get_package_name() == class.borrow().get_package_name();
        }
        if !self.is_private() {
            return c.borrow().get_package_name() == class.borrow().get_package_name();
        }
        class.eq(c)
    }

    /// Find the exception handler
    ///
    /// Returns -1 if not found
    pub fn find_exception_handler(&mut self, ex_class: &RcRefCell<Class>, pc: i64) -> i64 {
        if let Some(handler) = self
            .exception_table
            .as_mut()
            .unwrap()
            .find_exception_handler(ex_class, pc)
        {
            return handler.handler_pc();
        }

        -1
    }

    /// Get the exception stack line number
    ///
    /// Native method: -2
    ///
    /// Dynamic generated method(Or else): -1
    pub fn get_line_number(&self, pc: i64) -> i64 {
        if self.is_native() {
            return -2;
        }
        if self.line_number_table.is_none() {
            return -1;
        }
        self.line_number_table.as_ref().unwrap().get_line_number(pc)
    }

    fn new(class: RcRefCell<Class>, cf_method: &MemberInfo) -> RcRefCell<Self> {
        let (max_stack, max_locals, code, exception_table, line_number_table) =
            match cf_method.code_attribute() {
                Some(code_attr) => (
                    code_attr.max_stack(),
                    code_attr.max_locals(),
                    code_attr.code(),
                    Some(ExceptionTable::new(
                        code_attr.exception_table(),
                        &class.borrow().constant_pool(),
                    )),
                    code_attr.line_number_table_attribute(),
                ),
                None => (0, 0, vec![], None, None),
            };

        let mut method = Method {
            access_flags: cf_method.access_flags(),
            name: cf_method.name(),
            descriptor: cf_method.descriptor(),
            class: Some(class),
            max_stack,
            max_locals,
            code,
            arg_object_ref_count: 0,
            exception_table,
            line_number_table,
        };
        let parsed_descriptor = MethodDescriptorParser::parse(method.descriptor.clone());
        method.calc_arg_object_ref_count(&parsed_descriptor.parameter_types());
        if method.is_native() {
            method.inject_code_attribute(parsed_descriptor.return_type());
        }

        Rc::new(RefCell::new(method))
    }

    /// Inject bytecode and stack info for native method
    ///
    /// We use `0xfe` to execute native method
    ///
    /// Note that: `0xfe` is a reserved instruction for customize usage
    fn inject_code_attribute(&mut self, return_type: String) {
        // Todo: currently we set the stack to 4
        self.max_stack = 4;
        self.max_locals = self.arg_object_ref_count as u16;
        match return_type.as_bytes()[0] {
            b'L' | b'[' => {
                self.code = vec![0xfe, 0xb0]; // areturn
            }
            b'V' => {
                self.code = vec![0xfe, 0xb1]; // return
            }
            b'D' => {
                self.code = vec![0xfe, 0xaf]; // dreturn
            }
            b'F' => {
                self.code = vec![0xfe, 0xae]; // freturn
            }
            b'J' => {
                self.code = vec![0xfe, 0xad]; // lreturn
            }
            _ => {
                self.code = vec![0xfe, 0xac]; // ireturn
            }
        }
    }
}
