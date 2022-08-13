use super::super::{Log, Logger};
use super::{github_actions, Param};

pub fn print_title(param: Param) {
    let Param {
        program,
        arguments,
        syntax_highlight,
        support_github_action,
        ..
    } = param;

    if support_github_action {
        let method = github_actions::GroupOpening::from(syntax_highlight);
        Logger::new(&method, program, arguments).log();
    } else {
        Logger::new(&syntax_highlight, program, arguments).log();
    }
}
