pub mod args;
pub mod exec;
pub mod print_title;

use super::{github_actions, PrettyExec, SyntaxHighLight};
use structopt_utilities::StructOptUtils;

pub use std::process::ExitStatus;

pub struct Param<'a> {
    pub program: &'a str,
    pub arguments: &'a [String],
    pub skip_exec: bool,
    pub syntax_highlight: SyntaxHighLight<String>,
    pub support_github_action: bool,
}

pub fn main() -> Result<i32, String> {
    let args = args::Args::strict_from_args();
    let param = args.param();

    if param.skip_exec {
        print_title::print_title(param);
        return Ok(0);
    }

    exec::exec(param).and_then(|status| {
        status
            .code()
            .ok_or_else(|| "Failed to get status code".to_owned())
    })
}
