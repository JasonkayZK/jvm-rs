use std::cell::RefCell;
use std::rc::Rc;

use crate::classfile::member_info::MemberInfo;
use crate::instructions::base::bytecode_reader::BytecodeReader;
use crate::instructions::factory::new_instruction;
use crate::rtda::thread::Thread;

pub fn interpret(method_info: &MemberInfo) {
    if let Some(info) = method_info.code_attribute() {
        let thread_ref = Thread::new_ref();
        let frame = Thread::new_frame(
            thread_ref.clone(),
            info.max_locals() as usize,
            info.max_stack() as usize,
        );
        thread_ref.borrow_mut().push_frame(frame);

        interpret_loop(thread_ref, info.code());
    }
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
                inst.fetch_operands(&mut reader);
                frame.borrow_mut().set_next_pc(reader.pc() as i64);

                // Execute
                println!("pc: {}, inst:{:?}", pc, inst);
                inst.execute(&mut frame.borrow_mut());
            }
            Err(err) => {
                println!("LocalVars: {:?}", frame.borrow_mut().local_vars_mut());
                println!("OperandStack: {:?}", frame.borrow_mut().operand_stack_mut());
                panic!("{}", err);
            }
        }
    }
}
