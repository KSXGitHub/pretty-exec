use super::{Formatter, Logger, OsStr};

pub struct GroupOpening<Fmt: Formatter>(pub(crate) Fmt);
impl<Fmt: Formatter> Formatter for GroupOpening<Fmt> {
    type Output = String;
    fn fmt(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) -> String {
        format!("::group::{}", self.0.fmt(program, arguments))
    }
}
impl<Fmt: Formatter> Logger for GroupOpening<Fmt> {
    fn log(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) {
        println!("{}", self.fmt(program, arguments))
    }
}

pub struct GroupClosing;
impl Formatter for GroupClosing {
    type Output = String;
    fn fmt(&self, _: impl AsRef<OsStr>, _: &[impl AsRef<OsStr>]) -> String {
        "::endgroup::".to_owned()
    }
}
impl Logger for GroupClosing {
    fn log(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) {
        println!("{}", self.fmt(program, arguments))
    }
}
