use clap::{Args, Parser};

use crate::classpath::classpath_impl::ClasspathImpl;
use crate::classpath::entry::Entry;

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
    #[clap(long = "cp", required = true, help = "The classpath")]
    pub classpath: String,

    #[clap(name = "CLASS", required = true, help = "Main class name")]
    pub class: String,

    #[clap(name = "ARGS", help = "Arguments")]
    pub args: Vec<String>,
}

pub fn start_jvm(cp_args: &CpArgs, xjre_option: &Option<String>) {
    let mut cp = ClasspathImpl::parse(
        xjre_option.as_ref().unwrap_or(&String::new()),
        &cp_args.classpath,
    );

    println!(
        "classpath: {} class: {} args: {:?}",
        cp, cp_args.class, cp_args.args
    );

    let class_name = cp_args.class.replace('.', "/");
    let class_data = match cp.read_class(&class_name) {
        Ok(class_data) => class_data,
        Err(err) => {
            panic!(
                "Could not find or load main class {}: {}",
                cp_args.class, err
            );
        }
    };
    println!("class data: {:?}", class_data);
}
