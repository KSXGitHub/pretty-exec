use super::{Logger, OsStr};
pub struct Nothing;
impl<Program, Argument> Logger<Program, Argument> for Nothing
where
    Program: AsRef<OsStr>,
    Argument: AsRef<OsStr>,
{
    fn log(&self, _: Program, _: &[Argument]) {}
}
