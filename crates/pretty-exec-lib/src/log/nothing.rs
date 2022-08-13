use super::{Log, Logger};

pub struct Nothing;

impl<'a, Program: ?Sized, Arguments: ?Sized> Log for Logger<'a, Nothing, Program, Arguments> {
    fn log(&self) {}
}
