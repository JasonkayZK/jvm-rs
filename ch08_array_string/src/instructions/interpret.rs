use std::cell::RefCell;
use std::rc::Rc;

use log::info;

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instruction;
use crate::instructions::factory::new_instruction;
use crate::rtda::frame::Frame;
use crate::rtda::heap::method::Method;
use crate::rtda::thread::Thread;
use crate::types::RcRefCell;

pub fn interpret(method_ref: RcRefCell<Method>, enable_log: bool) {
    let thread_ref = Thread::new_ref();
    let frame = Thread::new_frame(thread_ref.clone(), method_ref);
    thread_ref.borrow_mut().push_frame(frame);

    interpret_loop(thread_ref, enable_log);
}

/// Interpret the instruction
///
/// The whole loop follows:
///
///  1. Calculate PC
///  2. Decode the instruction
///  3. Execute the instruction
fn interpret_loop(thread: Rc<RefCell<Thread>>, enable_log: bool) {
    let mut reader = BytecodeReader::default();

    loop {
        let frame = thread.borrow_mut().current_frame();
        let pc = frame.borrow().next_pc();

        thread.borrow_mut().set_pc(pc);

        // Decode
        reader.reset(frame.borrow().method().borrow().code(), pc as usize);

        let opcode = reader.read_u8();
        match new_instruction(opcode) {
            Ok(mut inst) => {
                inst.fetch_operands(&mut reader);
                frame.borrow_mut().set_next_pc(reader.pc() as i64);

                if enable_log {
                    log_instruction(&frame.borrow(), inst.as_ref());
                }
                inst.execute(&mut frame.borrow_mut());

                if thread.borrow().is_stack_empty() {
                    break;
                }
            }
            Err(err) => {
                log_frames(&thread);
                panic!("{}", err);
            }
        }
    }
}

fn log_instruction(frame: &Frame, inst: &dyn Instruction) {
    let method = frame.method();
    let method = method.borrow();
    let class = method.get_class();
    let class = class.borrow();
    let class_name = class.name();
    let method_name = method.name();
    let pc = frame.thread().borrow().pc();
    info!("{}.{} #{:2} {:?}", class_name, method_name, pc, inst);
}

fn log_frames(thread: &RcRefCell<Thread>) {
    while !thread.borrow().is_stack_empty() {
        let frame = thread.borrow_mut().pop_frame();
        let method = frame.as_ref().unwrap().borrow().method();
        let method = method.borrow();
        let class = method.get_class();
        let class = class.borrow();
        let pc = frame.as_ref().unwrap().borrow().next_pc();
        let class_name = class.name();
        info!(
            ">> pc: {:4} {}.{}{}",
            pc,
            class_name,
            method.name(),
            method.descriptor()
        );
    }
}
