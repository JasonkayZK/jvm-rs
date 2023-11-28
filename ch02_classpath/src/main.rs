use crate::cmd::{start_jvm, Cmd};
use clap::Parser;

mod classpath;
mod cmd;
mod error;

fn main() {
    let cmd = Cmd::parse();

    if let Some(cp_args) = cmd.cp {
        start_jvm(&cp_args, &cmd.xjre);
    }
}
