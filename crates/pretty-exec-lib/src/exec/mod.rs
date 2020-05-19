use super::{github_actions, PrettyExec, SyntaxHighLight};
use std::{env, io, process::Child};

pub use std::{ffi::OsStr, process::ExitStatus};

pub fn exec<Argument: AsRef<OsStr> + Copy>(arguments: &[Argument]) -> Result<ExitStatus, String> {
    let support_color = env::var("PRETTY_EXEC_NO_COLOR")
        .map(|value| value.to_lowercase() != "true")
        .unwrap_or(true);

    let support_github_action = env::var("PRETTY_EXEC_GITHUB_ACTION")
        .map(|value| value.to_lowercase() == "true")
        .unwrap_or(false);

    if arguments.is_empty() {
        return Err("No arguments".to_owned());
    }

    let program: Argument = arguments[0];
    let arguments: &[Argument] = &arguments[1..];
    let mut pretty_exec = PrettyExec::<_, Argument, _, _>::new(program);

    for argument in arguments {
        pretty_exec.arg(argument.to_owned());
    }

    let syntax_highlight = if support_color {
        SyntaxHighLight::default_color()
    } else {
        SyntaxHighLight::default_colorless()
    };

    let mut exec: Box<dyn FnMut() -> io::Result<Child>> = if support_github_action {
        let mut pretty_exec = pretty_exec
            .set_log_before(github_actions::GroupOpening(syntax_highlight))
            .set_log_after(github_actions::GroupClosing);
        Box::new(move || pretty_exec.spawn())
    } else {
        Box::new(move || pretty_exec.spawn())
    };

    let mut child: Child = exec().map_err(|error: io::Error| error.to_string())?;
    child.wait().map_err(|error| error.to_string())
}
