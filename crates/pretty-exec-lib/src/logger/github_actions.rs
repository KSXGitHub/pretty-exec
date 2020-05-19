use super::{Formatter, OsStr};

pub struct GroupOpening<Fmt: Formatter>(Fmt);
impl<Fmt: Formatter> Formatter for GroupOpening<Fmt> {
    fn fmt(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) -> String {
        format!("::group::{}", self.0.fmt(program, arguments))
    }
}

pub struct GroupClosing;
impl Formatter for GroupClosing {
    fn fmt(&self, _: impl AsRef<OsStr>, _: &[impl AsRef<OsStr>]) -> String {
        "::endgroup::".to_owned()
    }
}
