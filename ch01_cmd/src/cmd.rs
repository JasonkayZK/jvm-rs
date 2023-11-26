use clap::{Args, Parser};

#[derive(Debug, Parser)]
#[command(name = "java", version = "0.0.1")]
pub struct Cmd {
    #[clap(flatten)]
    pub cp: Option<CpArgs>,
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

pub fn start_jvm(cp_args: &CpArgs) {
    println!(
        "classpath: {} class: {} args: {:?}",
        cp_args.classpath, cp_args.class, cp_args.args
    );
}
