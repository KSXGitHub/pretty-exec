pub mod when;

pub use when::When;

use super::super::log::syntax_highlight::SyntaxHighLight;
use super::Param;
use clap::{Parser, ValueHint};
use is_terminal::IsTerminal;
use std::{ffi::OsString, io::stderr};

#[derive(Parser)]
#[clap(name = "pretty-exec", rename_all = "kebab", version)]
pub struct Args {
    /// Command to execute
    #[clap(name = "command", value_hint = ValueHint::CommandWithArguments, trailing_var_arg = true)]
    command: Vec<OsString>,

    /// Customize the prompt before the command.
    #[clap(long, default_value = "$")]
    prompt: String,

    /// Do not execute, print command only
    #[clap(long)]
    skip_exec: bool,

    /// When to use color
    #[clap(long, name = "color", value_enum, default_value_t = When::Auto)]
    color: When,

    /// Enable GitHub Action grouping
    #[clap(long)]
    github_actions: bool,
}

impl Args {
    pub fn syntax_highlight(&self) -> SyntaxHighLight {
        ColorMode::new(
            self.color,
            || self.github_actions,
            || stderr().is_terminal(),
        )
        .syntax_highlight()
    }

    pub fn param(&'_ self) -> Param<'_> {
        Param {
            program: self.command[0].as_os_str(),
            arguments: &self.command[1..],
            prompt: self.prompt.as_str(),
            skip_exec: self.skip_exec,
            support_github_action: self.github_actions,
            syntax_highlight: self.syntax_highlight(),
        }
    }
}

#[must_use]
#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
enum ColorMode {
    Colorless,
    Colorful,
}

impl ColorMode {
    fn new(
        color: When,
        github_actions: impl FnOnce() -> bool,
        is_terminal: impl FnOnce() -> bool,
    ) -> Self {
        match color {
            When::Never => return ColorMode::Colorless,
            When::Always => return ColorMode::Colorful,
            When::Auto => {}
        }

        if github_actions() || is_terminal() {
            return ColorMode::Colorful;
        }

        ColorMode::Colorless
    }

    fn syntax_highlight(self) -> SyntaxHighLight {
        match self {
            ColorMode::Colorless => SyntaxHighLight::colorless(),
            ColorMode::Colorful => SyntaxHighLight::colorful(),
        }
    }
}

#[cfg(test)]
mod test_color_mode {
    use super::{ColorMode, When};
    use itertools::Itertools;
    use maplit::btreemap;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn new() {
        use ColorMode::{Colorful, Colorless};
        use When::{Always, Auto, Never};

        let color = [Auto, Never, Always];
        let github_actions = [false, true];
        let is_terminal = [false, true];

        let received: BTreeMap<_, _> = [color.len(), github_actions.len(), is_terminal.len()]
            .into_iter()
            .map(|len| 0..len)
            .multi_cartesian_product()
            .map(|indices| (indices[0], indices[1], indices[2]))
            .map(|(i, j, k)| (color[i], github_actions[j], is_terminal[k]))
            .map(|(a, b, c)| ((a, b, c), ColorMode::new(a, || b, || c)))
            .collect();
        dbg!(&received);

        let expected = btreemap! {
            (Auto, false, false) => Colorless,
            (Auto, true, false) => Colorful,
            (Auto, false, true) => Colorful,
            (Auto, true, true) => Colorful,

            (Never, false, false) => Colorless,
            (Never, true, false) => Colorless,
            (Never, false, true) => Colorless,
            (Never, true, true) => Colorless,

            (Always, false, false) => Colorful,
            (Always, true, false) => Colorful,
            (Always, false, true) => Colorful,
            (Always, true, true) => Colorful,
        };

        assert_eq!(received, expected);
    }
}
