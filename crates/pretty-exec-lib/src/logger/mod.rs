pub mod concat;
pub mod github_actions;
pub mod nothing;
pub mod syntax_highlight;

pub use concat::*;
pub use nothing::*;
pub use std::ffi::OsStr;
pub use syntax_highlight::*;

pub trait Formatter {
    fn fmt(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) -> String;
}

pub trait Logger {
    fn log(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]);

    fn concat<A: Logger, B: Logger>(a: A, b: B) -> Concatenation<A, B> {
        Concatenation(a, b)
    }
}

impl<Fmt: Formatter> Logger for Fmt {
    fn log(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) {
        println!("{}", self.fmt(program, arguments));
    }
}
