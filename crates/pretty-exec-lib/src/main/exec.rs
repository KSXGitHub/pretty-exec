use super::{github_actions, ExitStatus, PrettyExec, SyntaxHighLight};
use std::io;

pub fn exec(
    arguments: &[String],
    support_color: bool,
    support_github_action: bool,
) -> Result<ExitStatus, String> {
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

    let mut exec: Box<dyn FnMut() -> io::Result<ExitStatus>> = if support_github_action {
        let mut pretty_exec = pretty_exec
            .set_log_before(github_actions::GroupOpening(syntax_highlight))
            .set_log_after(github_actions::GroupClosing);
        Box::new(move || pretty_exec.spawn())
    } else {
        let mut pretty_exec = pretty_exec.set_log_before(syntax_highlight);
        Box::new(move || pretty_exec.spawn())
    };

    exec().map_err(|error: io::Error| error.to_string())
}
