use super::{github_actions, PrettyExec, SyntaxHighLight};
use std::{env, io, process::Child};

pub use std::{ffi::OsStr, process::ExitStatus};

pub fn exec() -> Result<ExitStatus, String> {
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

    let program: &str = arguments[1].as_str();
    let arguments: &[String] = &arguments[2..];
    let mut pretty_exec = PrettyExec::new(program);

    for argument in arguments {
        pretty_exec.arg(argument);
    }

    let syntax_highlight = if support_color {
        SyntaxHighLight::default_color()
    } else {
        SyntaxHighLight::default_colorless()
    };
    let mut pretty_exec = pretty_exec.set_log_before(syntax_highlight);

    let mut exec: Box<dyn FnMut() -> io::Result<Child>> = if support_github_action {
        let mut pretty_exec = pretty_exec.set_log_after(github_actions::GroupClosing);
        Box::new(move || pretty_exec.spawn())
    } else {
        Box::new(move || pretty_exec.spawn())
    };

    let mut child: Child = exec().map_err(|error: io::Error| error.to_string())?;
    child.wait().map_err(|error| error.to_string())
}
