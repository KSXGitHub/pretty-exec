use super::{github_actions, Error, ExitStatus, Param, PrettyExec};

pub fn exec(param: Param) -> Result<ExitStatus, Error> {
    let Param {
        program,
        arguments,
        prompt,
        syntax_highlight,
        support_github_action,
        ..
    } = param;
    let pretty_exec = PrettyExec::new(prompt, program, arguments);

    let exec_result = if support_github_action {
        pretty_exec
            .set_log_before(github_actions::GroupOpening::from(syntax_highlight))
            .set_log_after(github_actions::GroupClosing)
            .spawn()
    } else {
        pretty_exec.set_log_before(syntax_highlight).spawn()
    };

    exec_result.map_err(Error::ExecutionError)
}
