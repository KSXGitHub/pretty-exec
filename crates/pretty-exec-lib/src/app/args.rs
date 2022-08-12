pub mod when;
pub use when::*;

use super::{Param, SyntaxHighLight};
use structopt::*;

#[derive(StructOpt)]
#[structopt(name = "pretty-exec", rename_all = "kebab")]
pub struct Args {
    /// Program to execute
    #[structopt(name = "program")]
    program: String,

    /// Arguments to pass to program
    #[structopt(name = "arguments")]
    arguments: Vec<String>,

    /// Do not execute, print command only
    #[structopt(long)]
    skip_exec: bool,

    /// When to use color
    #[structopt(long, default_value = "auto", possible_values = &["auto", "never", "always"])]
    color: When,

    /// Enable GitHub Action grouping
    #[structopt(long)]
    github_actions: bool,
}

impl Args {
    pub fn syntax_highlight(&self) -> SyntaxHighLight<String> {
        if self.color == When::Never {
            SyntaxHighLight::default_colorless()
        } else {
            SyntaxHighLight::default_color()
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
