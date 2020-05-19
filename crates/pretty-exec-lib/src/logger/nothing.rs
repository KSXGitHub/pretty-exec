use super::{Logger, OsStr};
pub struct Nothing;
impl Logger for Nothing {
    fn log(&self, _: impl AsRef<OsStr>, _: &[impl AsRef<OsStr>]) {}
}
