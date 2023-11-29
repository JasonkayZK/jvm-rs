use clap::Parser;

use crate::cmd::{start_jvm, Cmd};

#[allow(unused)]
pub mod classfile;
pub mod classpath;
pub mod cmd;

fn main() {
    let cmd = Cmd::parse();

    if let Some(cp_args) = cmd.cp {
        start_jvm(&cp_args, &cmd.xjre);
    }
}
