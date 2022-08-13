pub mod github_actions;
pub mod nothing;
pub mod syntax_highlight;

pub use nothing::Nothing;
pub use syntax_highlight::SyntaxHighLight;

#[must_use]
pub struct Logger<'a, Method: ?Sized, Program: ?Sized, Arguments: ?Sized> {
    pub method: &'a Method,
    pub program: &'a Program,
    pub arguments: &'a Arguments,
}

impl<'a, Method: ?Sized, Program: ?Sized, Arguments: ?Sized>
    Logger<'a, Method, Program, Arguments>
{
    pub fn new(method: &'a Method, program: &'a Program, arguments: &'a Arguments) -> Self {
        Logger {
            method,
            program,
            arguments,
        }
    }
}

pub trait Log {
    fn log(&self);
}
