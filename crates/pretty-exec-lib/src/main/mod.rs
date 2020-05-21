pub mod args;
pub mod exec;
pub mod print_title;

use super::{github_actions, PrettyExec, SyntaxHighLight};
use std::env;

pub use std::process::ExitStatus;

pub struct Param<'a> {
    pub program: &'a str,
    pub arguments: &'a [String],
    pub syntax_highlight: SyntaxHighLight<String>,
    pub support_github_action: bool,
}

pub fn main() -> Result<i32, String> {
    let arguments: Vec<_> = env::args().collect();

    let skip_exec = env::var("PRETTY_EXEC_SKIP_EXEC")
        .map(|value| value.to_lowercase() == "true")
        .unwrap_or(false);

    let support_color = env::var("PRETTY_EXEC_NO_COLOR")
        .map(|value| value.to_lowercase() != "true")
        .unwrap_or(true);

    let support_github_action = env::var("PRETTY_EXEC_GITHUB_ACTION")
        .map(|value| value.to_lowercase() == "true")
        .unwrap_or(false);

    if arguments.len() < 2 {
        return Err("No arguments".to_owned());
    }

    let program: &str = arguments[1].as_str();
    let arguments: &[String] = &arguments[2..];

    let syntax_highlight = if support_color {
        SyntaxHighLight::default_color()
    } else {
        SyntaxHighLight::default_colorless()
    };

    let param = Param {
        program,
        arguments,
        syntax_highlight,
        support_github_action,
    };

    if skip_exec {
        print_title::print_title(param);
        return Ok(0);
    }

    exec::exec(param).and_then(|status| {
        status
            .code()
            .ok_or_else(|| "Failed to get status code".to_owned())
    })
}
