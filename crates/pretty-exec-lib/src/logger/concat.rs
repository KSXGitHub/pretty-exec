use super::{Logger, OsStr};

pub struct Concatenation<A: Logger, B: Logger>(pub(crate) A, pub(crate) B);

impl<A: Logger, B: Logger> Logger for Concatenation<A, B> {
    fn log(&self, program: impl AsRef<OsStr>, arguments: &[impl AsRef<OsStr>]) {
        self.0.log(program.as_ref(), arguments.as_ref());
        self.1.log(program.as_ref(), arguments.as_ref());
    }
}
