pub mod when;

pub use when::*;

use super::{Param, SyntaxHighLight};
use clap::Parser;
use nu_ansi_term::AnsiGenericString;

#[derive(Parser)]
#[clap(name = "pretty-exec", rename_all = "kebab", version)]
pub struct Args {
    /// Program to execute
    #[clap(name = "program")]
    program: String,

    /// Arguments to pass to program
    #[clap(name = "arguments")]
    arguments: Vec<String>,

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
    pub fn syntax_highlight(&self) -> SyntaxHighLight<AnsiGenericString<'static, str>> {
        if self.color == When::Never {
            SyntaxHighLight::colorless()
        } else {
            SyntaxHighLight::colored()
        }
    }

    pub fn param(&'_ self) -> Param<'_> {
        Param {
            program: self.program.as_str(),
            arguments: self.arguments.as_ref(),
            skip_exec: self.skip_exec,
            support_github_action: self.github_actions,
            syntax_highlight: self.syntax_highlight(),
        }
    }
}
