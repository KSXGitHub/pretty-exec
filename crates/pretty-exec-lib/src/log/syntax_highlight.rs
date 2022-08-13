pub use nu_ansi_term as ansi_term;

use super::{Log, Logger};
use derive_more::Display;
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
#[derive(TypedBuilder)]
pub struct SyntaxHighLight<Prompt> {
    pub prompt: Prompt,
    #[builder(default)]
    pub program: Style,
    #[builder(default)]
    pub argument: Style,
    #[builder(default)]
    pub short_flag: Style,
    #[builder(default)]
    pub long_flag: Style,
}

impl SyntaxHighLight<&'static str> {
    const DEFAULT_PROMPT: &'static str = "$";
}

impl<Prompt> SyntaxHighLight<Prompt> {
    pub fn colorless() -> Self
    where
        ColorlessPrompt: Into<Prompt>,
    {
        let prompt = SyntaxHighLight::DEFAULT_PROMPT.pipe(ColorlessPrompt).into();
        SyntaxHighLight::builder().prompt(prompt).build()
    }
}

impl<Prompt> SyntaxHighLight<Prompt> {
    pub fn colorful() -> Self
    where
        ColorfulPrompt: Into<Prompt>,
    {
        let prompt = Style::default()
            .dimmed()
            .paint(SyntaxHighLight::DEFAULT_PROMPT)
            .pipe(ColorfulPrompt)
            .into();
        SyntaxHighLight::builder()
            .prompt(prompt)
            .program(Color::Green.into())
            .short_flag(Color::Red.into())
            .long_flag(Color::Red.into())
            .build()
    }
}

impl<'a, Prompt, Program: ?Sized, Arguments: ?Sized> Display
    for Logger<'a, SyntaxHighLight<Prompt>, Program, Arguments>
where
    Prompt: Display,
    &'a Program: AsRef<OsStr>,
    &'a Arguments: IntoIterator,
    <&'a Arguments as IntoIterator>::Item: AsRef<OsStr>,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Logger {
            method,
            program,
            arguments,
        } = self;

        let prompt = method.prompt.to_string();
        if !prompt.is_empty() {
            write!(f, "{prompt} ")?;
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
            let paint = |text: &str, style: Style| {
                text.to_owned()
                    .pipe(Cow::from)
                    .pipe(escape)
                    .pipe(|x| style.paint(x))
            };
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

impl<'a, Prompt, Program: ?Sized, Arguments: ?Sized> Log
    for Logger<'a, SyntaxHighLight<Prompt>, Program, Arguments>
where
    Prompt: Display,
    &'a Program: AsRef<OsStr>,
    &'a Arguments: IntoIterator,
    <&'a Arguments as IntoIterator>::Item: AsRef<OsStr>,
{
    fn log(&self) {
        eprintln!("{self}");
    }
}

#[derive(Display)]
pub struct ColorlessPrompt(&'static str);

#[derive(Display)]
pub struct ColorfulPrompt(AnsiGenericString<'static, str>);

impl From<ColorlessPrompt> for ColorfulPrompt {
    fn from(prompt: ColorlessPrompt) -> Self {
        let ColorlessPrompt(prompt) = prompt;
        prompt.pipe(AnsiGenericString::from).pipe(ColorfulPrompt)
    }
}
