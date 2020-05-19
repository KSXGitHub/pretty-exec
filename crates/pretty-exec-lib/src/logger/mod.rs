pub mod github_actions;
pub mod nothing;
pub mod syntax_highlight;

pub use nothing::*;
pub use std::ffi::OsStr;
pub use syntax_highlight::*;

pub trait Formatter {
    fn fmt(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) -> String;
}

pub trait Logger<Program: AsRef<OsStr>, Argument: AsRef<OsStr>> {
    fn log(&self, program: Program, arguments: &[Argument]);
}

impl<Fmt, Program, Argument> Logger<Program, Argument> for Fmt
where
    Fmt: Formatter,
    Program: AsRef<OsStr>,
    Argument: AsRef<OsStr>,
{
    fn log(&self, program: Program, arguments: &[Argument]) {
        println!("{}", self.fmt(program, arguments));
    }
}
