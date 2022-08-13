use super::super::{Log, Logger};
use derive_more::From;
use std::fmt::{self, Display, Formatter};

#[derive(From)]
pub struct GroupOpening<Fmt>(Fmt);

impl<'a, Fmt, Program: ?Sized, Arguments: ?Sized> Display
    for Logger<'a, GroupOpening<Fmt>, Program, Arguments>
where
    Logger<'a, Fmt, Program, Arguments>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let inner_logger = Logger::new(&self.method.0, self.program, self.arguments);
        write!(f, "::group::{inner_logger}")
    }
}

impl<'a, Fmt, Program: ?Sized, Arguments: ?Sized> Log
    for Logger<'a, GroupOpening<Fmt>, Program, Arguments>
where
    Logger<'a, Fmt, Program, Arguments>: Display,
{
    fn log(&self) {
        println!("{self}");
    }
}
