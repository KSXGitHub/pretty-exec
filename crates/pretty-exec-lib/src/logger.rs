use std::ffi::OsStr;

pub trait Logger<Program, Argument>
where
    Program: AsRef<OsStr>,
    Argument: AsRef<OsStr>,
{
    fn log(&self, program: Program, arguments: &[Argument]);
}
