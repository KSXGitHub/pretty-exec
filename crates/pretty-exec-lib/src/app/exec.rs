use super::{github_actions, Error, ExitStatus, Param, PrettyExec};

pub fn exec(param: Param) -> Result<ExitStatus, Error> {
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

    let exec_result = if support_github_action {
        pretty_exec
            .set_log_before(github_actions::GroupOpening::from(syntax_highlight))
            .set_log_after(github_actions::GroupClosing)
            .spawn()
    } else {
        pretty_exec.set_log_before(syntax_highlight).spawn()
    };

    exec_result.map_err(Error::from)
}
