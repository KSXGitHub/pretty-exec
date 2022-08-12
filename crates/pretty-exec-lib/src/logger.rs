pub mod github_actions;
pub mod nothing;
pub mod syntax_highlight;

pub use nothing::*;
pub use std::{ffi::OsStr, fmt::Display};
pub use syntax_highlight::*;

pub trait Formatter {
    type Output: Display;
    fn fmt(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) -> Self::Output;
}

pub trait Logger {
    fn log(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]);
}
