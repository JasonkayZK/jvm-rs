use std::cell::RefCell;
use std::rc::Rc;

use clap::{Args, Parser};
use log::error;

use crate::classpath::classpath_impl::ClasspathImpl;
use crate::instructions::interpret::interpret;
use crate::rtda::heap::class_loader::ClassLoader;

#[derive(Debug, Parser)]
#[command(name = "java", version = "0.0.1")]
pub struct Cmd {
    #[clap(long = "verbose:class",
    default_missing_value("true"),
    default_value("false"),
    num_args(0..=1),
    require_equals(true),
    action = clap::ArgAction::Set,
    help = "Enable verbose class output")]
    pub verbose_class_flag: bool,

    #[clap(long = "verbose:inst",
    default_missing_value("true"),
    default_value("false"),
    num_args(0..=1),
    require_equals(true),
    action = clap::ArgAction::Set,
    help = "Enable verbose instruction output")]
    pub verbose_inst_flag: bool,

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

pub fn start_jvm(cmd: Cmd) {
    let cp_args = match cmd.cp {
        None => {
            panic!("No classpath argument!")
        }
        Some(cp) => cp,
    };

    let cp = ClasspathImpl::parse(
        cmd.xjre.as_ref().unwrap_or(&String::new()),
        &cp_args.classpath,
    );

    let class_loader = Rc::new(RefCell::new(ClassLoader::new(cp, cmd.verbose_class_flag)));
    let class_name = cp_args.class.replace('.', "/");
    let main_class = class_loader
        .borrow_mut()
        .load_class(class_loader.clone(), class_name);
    let main_method = main_class.borrow_mut().get_main_method();
    match main_method {
        Some(member) => {
            interpret(member, cmd.verbose_inst_flag);
        }
        None => {
            error!("Main method not found in class {}", &cp_args.class);
        }
    }
}
