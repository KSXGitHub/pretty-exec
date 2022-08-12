pub mod github_actions;
pub mod nothing;
pub mod syntax_highlight;

pub use nothing::*;
pub use syntax_highlight::*;

use std::{ffi::OsStr, fmt::Display};

pub trait Format {
    type Output: Display;
    fn fmt(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) -> Self::Output;
}

pub trait Log {
    fn log(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]);
}
