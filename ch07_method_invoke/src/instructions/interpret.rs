use log::info;
use std::cell::RefCell;
use std::rc::Rc;

use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::factory::new_instruction;
use crate::rtda::heap::method::Method;
use crate::rtda::thread::Thread;
use crate::types::RcRefCell;

pub fn interpret(method_ref: RcRefCell<Method>) {
    let thread_ref = Thread::new_ref();
    let frame = Thread::new_frame(thread_ref.clone(), method_ref.clone());
    thread_ref.borrow_mut().push_frame(frame);

    interpret_loop(thread_ref, method_ref.borrow().code());
}

/// Interpret the instruction
///
/// The whole loop follows:
///
///  1. Calculate PC
///  2. Decode the instruction
///  3. Execute the instruction
fn interpret_loop(thread: Rc<RefCell<Thread>>, bytecode: Vec<u8>) {
    let frame = thread.borrow_mut().pop_frame().unwrap();
    let mut reader = BytecodeReader::default();

    loop {
        let pc = frame.borrow().next_pc();
        thread.borrow_mut().set_pc(pc);

        // Decode
        reader.reset(bytecode.clone(), pc as usize);
        let opcode = reader.read_u8();
        match new_instruction(opcode) {
            Ok(mut inst) => {
                let mut frame = frame.borrow_mut();

                inst.fetch_operands(&mut reader);
                frame.set_next_pc(reader.pc() as i64);

                // Execute
                info!("pc: {}, inst:{:?}", pc, inst);
                inst.execute(&mut frame);
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}
