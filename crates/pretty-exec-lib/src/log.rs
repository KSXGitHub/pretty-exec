pub mod github_actions;
pub mod nothing;
pub mod syntax_highlight;

pub use nothing::Nothing;
pub use syntax_highlight::SyntaxHighLight;

#[must_use]
pub struct Logger<'a, Method: ?Sized, Prompt: ?Sized, Program: ?Sized, Arguments: ?Sized> {
    pub method: &'a Method,
    pub prompt: &'a Prompt,
    pub program: &'a Program,
    pub arguments: &'a Arguments,
}

impl<'a, Method: ?Sized, Prompt: ?Sized, Program: ?Sized, Arguments: ?Sized>
    Logger<'a, Method, Prompt, Program, Arguments>
{
    pub fn new(
        method: &'a Method,
        prompt: &'a Prompt,
        program: &'a Program,
        arguments: &'a Arguments,
    ) -> Self {
        Logger {
            method,
            prompt,
            program,
            arguments,
        }
    }
}

pub trait Log {
    fn log(&self);
}
