use super::{Format, Log};
use derive_more::From;
use std::ffi::OsStr;

#[derive(From)]
pub struct GroupOpening<Fmt>(Fmt);

impl<Fmt: Format> Format for GroupOpening<Fmt> {
    type Output = String;
    fn fmt(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) -> String {
        format!("::group::{}", self.0.fmt(program, arguments))
    }
}

impl<Fmt: Format> Log for GroupOpening<Fmt> {
    fn log(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) {
        println!("{}", self.fmt(program, arguments))
    }
}

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
