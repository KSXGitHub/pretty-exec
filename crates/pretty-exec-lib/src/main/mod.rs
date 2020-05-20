mod exec;

use super::{github_actions, PrettyExec, SyntaxHighLight};
use std::env;

pub use std::process::ExitStatus;

pub(crate) struct Param {
    pub arguments: Vec<String>,
    pub syntax_highlight: SyntaxHighLight<String>,
    pub support_github_action: bool,
}

pub fn main() -> Result<i32, String> {
    let arguments: Vec<_> = env::args().collect();

    let support_color = env::var("PRETTY_EXEC_NO_COLOR")
        .map(|value| value.to_lowercase() != "true")
        .unwrap_or(true);

    let support_github_action = env::var("PRETTY_EXEC_GITHUB_ACTION")
        .map(|value| value.to_lowercase() == "true")
        .unwrap_or(false);

    if arguments.len() < 2 {
        return Err("No arguments".to_owned());
    }

    let syntax_highlight = if support_color {
        SyntaxHighLight::default_color()
    } else {
        SyntaxHighLight::default_colorless()
    };

    exec::exec(Param {
        arguments,
        syntax_highlight,
        support_github_action,
    })
    .and_then(|status| {
        status
            .code()
            .ok_or_else(|| "Failed to get status code".to_owned())
    })
}
