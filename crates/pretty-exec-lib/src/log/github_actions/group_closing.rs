use super::super::{Format, Log};
use std::ffi::OsStr;

pub struct GroupClosing;

impl Format for GroupClosing {
    type Output = &'static str;
    fn fmt(&self, _: impl AsRef<OsStr>, _: &[impl AsRef<OsStr>]) -> &'static str {
        "::endgroup::"
    }
}

impl Log for GroupClosing {
    fn log(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) {
        println!("{}", self.fmt(program, arguments))
    }
}
