use clap::{Args, Parser};

use crate::classfile::class_file::ClassFile;
use crate::classfile::member_info::MemberInfo;
use crate::classpath::classpath_impl::ClasspathImpl;
use crate::classpath::entry::Entry;
use crate::instructions::interpret::interpret;

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
    let mut cp = ClasspathImpl::parse(
        xjre_option.as_ref().unwrap_or(&String::new()),
        &cp_args.classpath,
    );

    println!(
        "classpath: {} class: {} args: {:?}",
        cp, cp_args.class, cp_args.args
    );

    let class_name = cp_args.class.replace('.', "/");
    let class_file = load_class(&class_name, &mut cp);
    match get_main_method(&class_file) {
        Some(member_info) => {
            interpret(member_info);
        }
        None => {
            println!("Main method not found in class {}", &cp_args.class);
        }
    }
}

fn load_class(class_name: &str, class_path: &mut ClasspathImpl) -> ClassFile {
    let class_data = match class_path.read_class(class_name) {
        Ok(class_data) => class_data,
        Err(err) => {
            panic!("Could not find or load main class {}: {}", class_name, err);
        }
    };

    match ClassFile::parse(class_data) {
        Ok(class_file) => class_file,
        Err(err) => panic!("{}", err),
    }
}

fn get_main_method(cf: &ClassFile) -> Option<&MemberInfo> {
    cf.methods()
        .iter()
        .find(|&m| m.name() == "main" && m.descriptor() == "([Ljava/lang/String;)V")
}
