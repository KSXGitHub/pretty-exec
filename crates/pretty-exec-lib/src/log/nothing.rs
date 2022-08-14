use super::{Log, Logger};

#[must_use]
pub struct Nothing;

impl<'a, Prompt: ?Sized, Program: ?Sized, Arguments: ?Sized> Log
    for Logger<'a, Nothing, Prompt, Program, Arguments>
{
    fn log(&self) {}
}
