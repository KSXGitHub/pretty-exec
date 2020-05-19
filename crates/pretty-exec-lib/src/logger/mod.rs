pub mod syntax_highlight;

pub use std::ffi::OsStr;
pub use syntax_highlight::*;

pub trait Logger {
    fn fmt(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) -> String;

    fn log(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) {
        println!("{}", self.fmt(program, arguments));
    }
}
