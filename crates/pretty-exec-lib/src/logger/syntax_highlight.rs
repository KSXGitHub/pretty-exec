use super::{Formatter, OsStr};
use std::fmt::{Display, Write};

pub use ansi_term::{Color, Style};

pub struct SyntaxHighLight<Prompt: Display> {
    prompt: Prompt,
    program: Style,
    argument: Style,
    short_flag: Style,
    long_flag: Style,
}

impl SyntaxHighLight<String> {
    const DEFAULT_PROMPT: &'static str = "$";

    pub fn default_colorless() -> Self {
        Self {
            prompt: Self::DEFAULT_PROMPT.to_owned(),
            program: Style::default(),
            argument: Style::default(),
            short_flag: Style::default(),
            long_flag: Style::default(),
        }
    }

    pub fn default_color() -> Self {
        Self {
            prompt: Style::default()
                .dimmed()
                .paint(Self::DEFAULT_PROMPT)
                .to_string(),
            program: Color::Green.into(),
            argument: Style::default(),
            short_flag: Color::Red.into(),
            long_flag: Color::Red.into(),
        }
    }
}

impl<Prompt: Display> Formatter for SyntaxHighLight<Prompt> {
    fn fmt(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) -> String {
        use shell_escape::escape;
        let mut result = String::new();

        let prompt = format!("{}", self.prompt);
        if !prompt.is_empty() {
            write!(result, "{} ", prompt).expect("write prompt");
        }

        write!(
            result,
            "{}",
            self.program
                .paint(escape(program.as_ref().to_string_lossy()))
        )
        .expect("write program name");

        for argument in arguments {
            let argument = escape(argument.as_ref().to_string_lossy());
            let argument = if argument.starts_with("--") {
                let segments: Vec<_> = argument.splitn(2, '=').collect();
                match segments[..] {
                    [_] => self.long_flag.paint(argument),
                    [flag, val] => Style::default().paint(format!(
                        "{flag}{eq}{val}",
                        flag = self.long_flag.paint(flag),
                        eq = self.argument.paint("="),
                        val = self.argument.paint(val),
                    )),
                    _ => unreachable!(),
                }
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
