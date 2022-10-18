pub mod args;
pub mod exec;
pub mod print_title;

use super::{github_actions, log::syntax_highlight::SyntaxHighLight, Error, PrettyExec};
use clap::Parser;
use pipe_trait::Pipe;
use std::{
    ffi::{OsStr, OsString},
    process::ExitStatus,
};

pub struct Param<'a> {
    pub program: &'a OsStr,
    pub arguments: &'a [OsString],
    pub prompt: &'a str,
    pub skip_exec: bool,
    pub syntax_highlight: SyntaxHighLight,
    pub support_github_action: bool,
}

pub fn main() -> Result<i32, Error> {
    let args = args::Args::parse();
    let param = args.param()?;

    if param.skip_exec {
        print_title::print_title(param);
        return Ok(0);
    }

    exec::exec(param)?
        .pipe_ref(ExitStatus::code)
        .ok_or(Error::StatusCodeAcquisitionFailure)
}
