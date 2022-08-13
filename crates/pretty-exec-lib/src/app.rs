pub mod args;
pub mod exec;
pub mod print_title;

use super::{
    github_actions,
    log::syntax_highlight::{ColorfulPrompt, SyntaxHighLight},
    Error, PrettyExec,
};
use clap::Parser;
use std::process::ExitStatus;

pub struct Param<'a> {
    pub program: &'a str,
    pub arguments: &'a [String],
    pub skip_exec: bool,
    pub syntax_highlight: SyntaxHighLight<ColorfulPrompt>,
    pub support_github_action: bool,
}

pub fn main() -> Result<i32, Error> {
    let args = args::Args::parse();
    let param = args.param();

    if param.skip_exec {
        print_title::print_title(param);
        return Ok(0);
    }

    exec::exec(param).and_then(|status| status.code().ok_or(Error::StatusCodeAcquisitionFailure))
}
