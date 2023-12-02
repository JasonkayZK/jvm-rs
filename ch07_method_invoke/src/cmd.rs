use std::cell::RefCell;
use std::rc::Rc;

use clap::{Args, Parser};
use log::{error, info, warn};

use crate::classpath::classpath_impl::ClasspathImpl;
use crate::instructions::interpret::interpret;
use crate::rtda::heap::class_loader::ClassLoader;

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

pub fn start_jvm(cp_args: &CpArgs, xjre_option: &Option<String>) {
    let cp = ClasspathImpl::parse(
        xjre_option.as_ref().unwrap_or(&String::new()),
        &cp_args.classpath,
    );

    info!(
        "classpath: {} class: {} args: {:?}",
        cp, cp_args.class, cp_args.args
    );

    let class_loader = Rc::new(RefCell::new(ClassLoader::new(cp)));
    let class_name = cp_args.class.replace('.', "/");
    let main_class = class_loader
        .borrow_mut()
        .load_class(class_loader.clone(), class_name);
    let main_method = main_class.borrow_mut().get_main_method();
    match main_method {
        Some(member) => {
            interpret(member);
        }
        None => {
            error!("Main method not found in class {}", &cp_args.class);
        }
    }
}
