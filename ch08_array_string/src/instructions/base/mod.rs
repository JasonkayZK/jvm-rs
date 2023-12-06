use std::fmt::Debug;

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::rtda::frame::Frame;
use crate::rtda::heap::class::Class;
use crate::rtda::heap::method::Method;
use crate::rtda::thread::Thread;
use crate::types::RcRefCell;

pub mod bytecode_reader;

pub trait Instruction: Debug {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        // Default operation does nothing
    }

    /// Execute the current frame from stack
    fn execute(&mut self, frame: &mut Frame);
}

pub fn branch(frame: &mut Frame, offset: i64) {
    let pc = frame.thread().borrow().pc();
    let next_pc = pc + offset;
    frame.set_next_pc(next_pc);
}

pub fn invoke_method(frame: &mut Frame, method: &RcRefCell<Method>) {
    let thread = frame.thread();
    let mut new_frame = Thread::new_frame(thread.clone(), method.clone());

    let arg_object_ref_count = method.borrow().arg_object_ref_count() as i32;
    if arg_object_ref_count > 0 {
        let mut i = arg_object_ref_count - 1;
        while i >= 0 {
            let obj_ref = frame.operand_stack_mut().pop_var();
            new_frame.local_vars_mut().set_var(i as usize, obj_ref);
            i -= 1;
        }
    }

    thread.borrow_mut().push_frame(new_frame);

    // todo
    // Hack!
    if method.borrow().is_native() {
        if method.borrow().name() == "registerNatives" {
            thread.borrow_mut().pop_frame();
        } else {
            panic!(
                "native method: {}.{}{}",
                method.borrow().get_class().borrow().name(),
                method.borrow().name(),
                method.borrow().descriptor()
            );
        }
    }
}

pub fn init_class(thread: &RcRefCell<Thread>, class: &RcRefCell<Class>) {
    class.borrow_mut().start_init();
    schedule_clinit(thread, class);
    init_super_class(thread, class);
}

pub fn schedule_clinit(thread: &RcRefCell<Thread>, class: &RcRefCell<Class>) {
    let clinit = class.borrow().get_clinit_method();
    if let Some(clinit) = clinit {
        // exec <clinit>
        let new_frame = Thread::new_frame(thread.clone(), clinit);
        thread.borrow_mut().push_frame(new_frame);
    }
}

pub fn init_super_class(thread: &RcRefCell<Thread>, class: &RcRefCell<Class>) {
    if !class.borrow().is_interface() {
        let super_class = class.borrow().super_class();
        if super_class.is_some() && !super_class.as_ref().unwrap().borrow().init_started() {
            init_class(thread, super_class.as_ref().unwrap());
        }
    }
}
