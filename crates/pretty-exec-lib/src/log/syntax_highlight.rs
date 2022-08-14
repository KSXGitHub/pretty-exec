pub use nu_ansi_term as ansi_term;

use super::{Log, Logger};
use nu_ansi_term::{AnsiGenericString, Color, Style};
use pipe_trait::Pipe;
use shell_escape::unix::escape;
use std::{
    borrow::Cow,
    ffi::OsStr,
    fmt::{self, Display, Formatter},
};
use typed_builder::TypedBuilder;

#[must_use]
#[derive(Default, TypedBuilder)]
pub struct SyntaxHighLight {
    #[builder(default)]
    pub prompt: Style,
    #[builder(default)]
    pub program: Style,
    #[builder(default)]
    pub argument: Style,
    #[builder(default)]
    pub short_flag: Style,
    #[builder(default)]
    pub long_flag: Style,
}

impl SyntaxHighLight {
    pub fn colorless() -> Self {
        Default::default()
    }

    pub fn colorful() -> Self {
        SyntaxHighLight::builder()
            .prompt(Style::new().dimmed())
            .program(Color::Green.into())
            .short_flag(Color::Red.into())
            .long_flag(Color::Red.into())
            .build()
    }
}

impl<'a, Prompt: ?Sized, Program: ?Sized, Arguments: ?Sized> Display
    for Logger<'a, SyntaxHighLight, Prompt, Program, Arguments>
where
    &'a Prompt: AsRef<str>,
    &'a Program: AsRef<OsStr>,
    &'a Arguments: IntoIterator,
    <&'a Arguments as IntoIterator>::Item: AsRef<OsStr>,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Logger {
            method,
            prompt,
            program,
            arguments,
        } = self;

        let prompt = prompt.as_ref();
        if !prompt.is_empty() {
            write!(f, "{} ", method.prompt.paint(prompt))?;
        }

        write!(
            f,
            "{}",
            program
                .as_ref()
                .to_string_lossy()
                .pipe(escape)
                .pipe(|x| method.program.paint(x))
        )?;

        for argument in *arguments {
            let argument = argument.as_ref().to_string_lossy();
            fn paint(text: &str, style: Style) -> AnsiGenericString<'_, str> {
                text.pipe(Cow::from)
                    .pipe(escape)
                    .pipe(|text| style.paint(text))
            }
            let argument = if argument.starts_with("--") {
                let segments: Vec<_> = argument.splitn(2, '=').collect();
                match segments[..] {
                    [_] => paint(&argument, method.long_flag),
                    [flag, val] => Style::default().paint(format!(
                        "{flag}{eq}{val}",
                        flag = paint(flag, method.long_flag),
                        eq = method.argument.paint("="),
                        val = paint(val, method.argument),
                    )),
                    _ => unreachable!(),
                }
            } else if argument.starts_with('-') {
                paint(&argument, method.short_flag)
            } else {
                paint(&argument, method.argument)
            };
            write!(f, " {argument}")?;
        }

        Ok(())
    }
}

impl<'a, Prompt: ?Sized, Program: ?Sized, Arguments: ?Sized> Log
    for Logger<'a, SyntaxHighLight, Prompt, Program, Arguments>
where
    &'a Prompt: AsRef<str>,
    &'a Program: AsRef<OsStr>,
    &'a Arguments: IntoIterator,
    <&'a Arguments as IntoIterator>::Item: AsRef<OsStr>,
{
    fn log(&self) {
        eprintln!("{self}");
    }
}
