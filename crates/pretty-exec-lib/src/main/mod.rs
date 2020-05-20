mod exec;

use super::{github_actions, PrettyExec, SyntaxHighLight};
use std::env;

pub use std::process::ExitStatus;

pub(crate) struct Param {
    pub arguments: Vec<String>,
    pub support_color: bool,
    pub support_github_action: bool,
}

pub fn main() -> Result<ExitStatus, String> {
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

    exec::exec(Param {
        arguments,
        support_color,
        support_github_action,
    })
}
