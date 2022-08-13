use super::super::{Log, Logger};
use std::fmt::{self, Display, Formatter};

pub struct GroupClosing;

impl<'a, Program: ?Sized, Arguments: ?Sized> Display
    for Logger<'a, GroupClosing, Program, Arguments>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "::endgroup::")
    }
}

impl<'a, Program: ?Sized, Arguments: ?Sized> Log for Logger<'a, GroupClosing, Program, Arguments> {
    fn log(&self) {
        println!("{self}");
    }
}
