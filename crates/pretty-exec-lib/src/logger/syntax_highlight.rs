use super::{Formatter, OsStr};
use std::fmt::{Display, Write};

pub use ansi_term::Style;

pub struct SyntaxHighLight<Prompt: Display> {
    prompt: Prompt,
    program: Style,
    argument: Style,
    short_flag: Style,
    long_flag: Style,
}

impl<Prompt: Display> Formatter for SyntaxHighLight<Prompt> {
    fn fmt(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) -> String {
        let mut result = String::new();

        let prompt = format!("{}", self.prompt);
        if !prompt.is_empty() {
            write!(result, "{} ", prompt).expect("write prompt");
        }

        write!(
            result,
            "{}",
            self.program.paint(program.as_ref().to_string_lossy())
        )
        .expect("write program name");

        for argument in arguments {
            let argument = argument.as_ref().to_string_lossy();
            let argument = if argument.starts_with("--") {
                self.long_flag.paint(argument)
            } else if argument.starts_with('-') {
                self.short_flag.paint(argument)
            } else {
                self.argument.paint(argument)
            };
            write!(result, " {}", argument).expect("write argument");
        }

        result
    }
}
