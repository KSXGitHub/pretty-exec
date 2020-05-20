use super::super::*;
use super::{github_actions, Param};

pub fn print_title(param: Param) {
    let Param {
        arguments,
        syntax_highlight,
        support_github_action,
    } = param;

    let program: &str = arguments[1].as_str();
    let arguments: &[String] = &arguments[2..];

    if support_github_action {
        github_actions::GroupOpening(syntax_highlight).log(program, arguments);
    } else {
        syntax_highlight.log(program, arguments);
    }
}
