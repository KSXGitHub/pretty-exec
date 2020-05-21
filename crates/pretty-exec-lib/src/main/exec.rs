use super::{github_actions, ExitStatus, Param, PrettyExec};
use std::io;

pub fn exec(param: Param) -> Result<ExitStatus, String> {
    let Param {
        program,
        arguments,
        syntax_highlight,
        support_github_action,
        ..
    } = param;
    let mut pretty_exec = PrettyExec::new(program);

    for argument in arguments {
        pretty_exec.arg(argument);
    }

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
