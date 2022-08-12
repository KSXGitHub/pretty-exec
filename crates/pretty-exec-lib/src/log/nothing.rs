use super::{Log, OsStr};

pub struct Nothing;

impl Log for Nothing {
    fn log(&self, _: impl AsRef<OsStr>, _: &[impl AsRef<OsStr>]) {}
}
