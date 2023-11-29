use clap::{Args, Parser};

use crate::classfile::class_file::ClassFile;
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
    let class_file = load_class(&class_name, &mut cp);
    print_class_info(&class_file);
}

fn load_class(class_name: &str, class_path: &mut ClasspathImpl) -> ClassFile {
    let class_data = match class_path.read_class(&class_name) {
        Ok(class_data) => class_data,
        Err(err) => {
            panic!("Could not find or load main class {}: {}", class_name, err);
        }
    };

    let class_file = match ClassFile::parse(class_data) {
        Ok(class_file) => class_file,
        Err(err) => panic!("{}", err),
    };

    class_file
}

fn print_class_info(class_file: &ClassFile) {
    println!(
        "version: {}.{}",
        class_file.major_version(),
        class_file.minor_version()
    );
    println!(
        "constants count: {}",
        class_file.constant_pool().borrow().infos.len()
    );
    println!("access flags: 0x{:x}", class_file.access_flags());
    println!("this class: {}", class_file.class_name());
    println!("super class: {}", class_file.super_class_name());
    println!("interfaces: {:?}", class_file.interface_names());
    println!("fields count: {:?}", class_file.fields().len());
    for field in class_file.fields() {
        println!(" {}", field.name());
    }
    println!("methods count: {:?}", class_file.methods().len());
    for method in class_file.methods() {
        println!(" {}", method.name());
    }
}
