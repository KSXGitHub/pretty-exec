use super::super::{Log, Logger};
use std::fmt::{self, Display, Formatter};

#[must_use]
pub struct GroupClosing;

impl<'a, Prompt: ?Sized, Program: ?Sized, Arguments: ?Sized> Display
    for Logger<'a, GroupClosing, Prompt, Program, Arguments>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "::endgroup::")
    }
}

impl<'a, Prompt: ?Sized, Program: ?Sized, Arguments: ?Sized> Log
    for Logger<'a, GroupClosing, Prompt, Program, Arguments>
{
    fn log(&self) {
        println!("{self}");
    }
}
