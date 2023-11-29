use clap::{Args, Parser};

use crate::rtda::frame::Frame;
use crate::rtda::local_var::LocalVar;
use crate::rtda::operand_stack::OperandStack;

#[derive(Debug, Parser)]
#[command(name = "java", version = "0.0.1")]
pub struct Cmd {
    #[clap(flatten)]
    pub cp: Option<CpArgs>,

    #[arg(long, required = false, help = "Path to jre")]
    pub xjre: Option<String>,
}

#[derive(Args, Debug)]
pub struct CpArgs {
    #[clap(
        long = "cp",
        required = false,
        help = "The classpath",
        default_value = ""
    )]
    pub classpath: String,

    #[clap(name = "CLASS", required = false, help = "Main class name")]
    pub class: String,

    #[clap(name = "ARGS", help = "Arguments")]
    pub args: Vec<String>,
}

pub fn start_jvm(_cp_args: &CpArgs, _xjre_option: &Option<String>) {
    let mut frame = Frame::new(100, 100);
    test_local_vars(frame.local_vars_mut());
    test_operand_stack(frame.operand_stack_mut());
}

fn test_local_vars(local_vars: &mut LocalVar) {
    local_vars.set_int(0, 100);
    local_vars.set_int(1, -100);
    local_vars.set_long(2, 2997924580);
    local_vars.set_long(4, -2997924580);
    local_vars.set_float(6, std::f32::consts::PI);
    local_vars.set_double(7, std::f64::consts::E);
    local_vars.set_ref(9, None);

    println!("{}", local_vars.get_int(0));
    println!("{}", local_vars.get_int(1));
    println!("{}", local_vars.get_long(2));
    println!("{}", local_vars.get_long(4));
    println!("{}", local_vars.get_float(6));
    println!("{}", local_vars.get_double(7));
    println!("{:?}", local_vars.get_ref(9));
}

fn test_operand_stack(operand_stack: &mut OperandStack) {
    operand_stack.push_int(100);
    operand_stack.push_int(-100);
    operand_stack.push_long(2997924580);
    operand_stack.push_long(-2997924580);
    operand_stack.push_float(std::f32::consts::PI);
    operand_stack.push_double(std::f64::consts::E);
    operand_stack.push_ref(None);

    println!("{:?}", operand_stack.pop_ref());
    println!("{}", operand_stack.pop_double());
    println!("{}", operand_stack.pop_float());
    println!("{}", operand_stack.pop_long());
    println!("{}", operand_stack.pop_long());
    println!("{}", operand_stack.pop_int());
    println!("{}", operand_stack.pop_int());
}
