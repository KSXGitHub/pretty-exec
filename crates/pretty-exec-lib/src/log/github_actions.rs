use super::{Format, Log, OsStr};
use derive_more::From;

#[derive(From)]
pub struct GroupOpening<Fmt: Format>(Fmt);
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
    type Output = String;
    fn fmt(&self, _: impl AsRef<OsStr>, _: &[impl AsRef<OsStr>]) -> String {
        "::endgroup::".to_owned()
    }
}
impl Log for GroupClosing {
    fn log(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) {
        println!("{}", self.fmt(program, arguments))
    }
}
