use super::super::*;
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
        github_actions::GroupOpening::from(syntax_highlight).log(program, arguments);
    } else {
        syntax_highlight.log(program, arguments);
    }
}
