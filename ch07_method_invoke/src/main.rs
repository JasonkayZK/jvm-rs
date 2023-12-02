#![allow(unused)]

use clap::Parser;

use crate::cmd::{start_jvm, Cmd};
use crate::logger::LogLevel;

pub mod classfile;
pub mod classpath;
pub mod cmd;
pub mod instructions;
pub mod logger;
pub mod rtda;
pub mod types;

fn main() {
    let cmd = Cmd::parse();
    logger::init(Some(LogLevel::Debug));

    if let Some(cp_args) = cmd.cp {
        start_jvm(&cp_args, &cmd.xjre);
    }
}
