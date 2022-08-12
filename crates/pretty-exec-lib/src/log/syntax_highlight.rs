pub use nu_ansi_term as ansi_term;

use super::{Format, Log, OsStr};
use nu_ansi_term::{Color, Style};
use std::fmt::{Display, Write};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct SyntaxHighLight<Prompt: Display> {
    prompt: Prompt,
    #[builder(default)]
    program: Style,
    #[builder(default)]
    argument: Style,
    #[builder(default)]
    short_flag: Style,
    #[builder(default)]
    long_flag: Style,
}

impl SyntaxHighLight<String> {
    const DEFAULT_PROMPT: &'static str = "$";

    pub fn colorless() -> Self {
        SyntaxHighLight::builder()
            .prompt(SyntaxHighLight::DEFAULT_PROMPT.to_string())
            .build()
    }

    pub fn colored() -> Self {
        let prompt = Style::default()
            .dimmed()
            .paint(Self::DEFAULT_PROMPT)
            .to_string();
        SyntaxHighLight::builder()
            .prompt(prompt)
            .program(Color::Green.into())
            .short_flag(Color::Red.into())
            .long_flag(Color::Red.into())
            .build()
    }
}

impl<Prompt: Display> Format for SyntaxHighLight<Prompt> {
    type Output = String;

    fn fmt(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) -> String {
        use pipe_trait::*;
        use shell_escape::unix::escape;
        let mut result = String::new();

        let prompt = format!("{}", self.prompt);
        if !prompt.is_empty() {
            write!(result, "{} ", prompt).expect("write prompt");
        }

        write!(
            result,
            "{}",
            program
                .as_ref()
                .to_string_lossy()
                .pipe(escape)
                .pipe(|x| self.program.paint(x))
        )
        .expect("write program name");

        for argument in arguments {
            let argument = argument.as_ref().to_string_lossy();
            let paint = |text: &str, style: &Style| {
                text.to_owned()
                    .pipe(std::borrow::Cow::from)
                    .pipe(escape)
                    .pipe(|x| style.paint(x))
            };
            let argument = if argument.starts_with("--") {
                let segments: Vec<_> = argument.splitn(2, '=').collect();
                match segments[..] {
                    [_] => paint(&argument, &self.long_flag),
                    [flag, val] => Style::default().paint(format!(
                        "{flag}{eq}{val}",
                        flag = paint(flag, &self.long_flag),
                        eq = self.argument.paint("="),
                        val = paint(val, &self.argument),
                    )),
                    _ => unreachable!(),
                }
            } else if argument.starts_with('-') {
                paint(&argument, &self.short_flag)
            } else {
                paint(&argument, &self.argument)
            };
            write!(result, " {}", argument).expect("write argument");
        }

        result
    }
}

impl<Prompt: Display> Log for SyntaxHighLight<Prompt> {
    fn log(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) {
        eprintln!("{}", self.fmt(program, arguments))
    }
}
