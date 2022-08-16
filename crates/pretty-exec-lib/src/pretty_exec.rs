use super::log::{Log, Logger, Nothing};
use std::{
    ffi::{OsStr, OsString},
    io,
    process::{Command, ExitStatus},
};

pub struct PrettyExec<'a, PreLog, PostLog> {
    prompt: &'a str,
    program: &'a OsStr,
    arguments: &'a [OsString],
    log_before: PreLog,
    log_after: PostLog,
}

impl<'a, PreLog, PostLog> PrettyExec<'a, PreLog, PostLog> {
    pub fn spawn(&'a mut self) -> io::Result<ExitStatus>
    where
        Logger<'a, PreLog, str, OsStr, [OsString]>: Log,
        Logger<'a, PostLog, str, OsStr, [OsString]>: Log,
    {
        Logger::new(&self.log_before, self.prompt, self.program, self.arguments).log();
        let result = Command::new(self.program)
            .args(self.arguments)
            .spawn()?
            .wait();
        Logger::new(&self.log_after, self.prompt, self.program, self.arguments).log();
        result
    }

    pub fn set_log_before<Logger>(self, log_before: Logger) -> PrettyExec<'a, Logger, PostLog> {
        PrettyExec {
            prompt: self.prompt,
            program: self.program,
            arguments: self.arguments,
            log_after: self.log_after,
            log_before,
        }
    }

    pub fn set_log_after<Logger>(self, log_after: Logger) -> PrettyExec<'a, PreLog, Logger> {
        PrettyExec {
            prompt: self.prompt,
            program: self.program,
            arguments: self.arguments,
            log_before: self.log_before,
            log_after,
        }
    }
}

impl<'a> PrettyExec<'a, Nothing, Nothing> {
    pub fn new(prompt: &'a str, program: &'a OsStr, arguments: &'a [OsString]) -> Self {
        PrettyExec {
            prompt,
            program,
            arguments,
            log_before: Nothing,
            log_after: Nothing,
        }
    }
}
