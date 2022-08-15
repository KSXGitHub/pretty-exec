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
            fn paint_escape(text: &str, style: Style) -> AnsiGenericString<'_, str> {
                text.pipe(Cow::from)
                    .pipe(escape)
                    .pipe(|text| style.paint(text))
            }
            if argument.starts_with("--") {
                let mut segments = argument.splitn(2, '=');
                match (segments.next(), segments.next()) {
                    (Some(_), None) => write!(f, " {}", paint_escape(&argument, method.long_flag))?,
                    (Some(flag), Some(val)) => write!(
                        f,
                        " {flag}{eq}{val}",
                        flag = paint_escape(flag, method.long_flag),
                        eq = method.argument.paint("="),
                        val = paint_escape(val, method.argument),
                    )?,
                    _ => unreachable!(),
                }
            } else if argument.starts_with('-') {
                write!(f, " {}", paint_escape(&argument, method.short_flag))?
            } else {
                write!(f, " {}", paint_escape(&argument, method.argument))?
            };
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
