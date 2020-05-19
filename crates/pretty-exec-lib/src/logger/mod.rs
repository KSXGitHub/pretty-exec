pub mod syntax_highlight;

pub use std::ffi::OsStr;
pub use syntax_highlight::*;

pub trait Logger {
    fn log(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]);
}
