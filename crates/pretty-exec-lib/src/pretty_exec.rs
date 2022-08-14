use super::log::{Log, Logger, Nothing};
use std::{
    ffi::OsStr,
    io,
    process::{Command, ExitStatus},
};

pub struct PrettyExec<'a, PreLog, PostLog> {
    prompt: &'a str,
    program: &'a OsStr,
    arguments: Vec<&'a OsStr>,
    command: Command,
    log_before: PreLog,
    log_after: PostLog,
}

impl<'a, PreLog, PostLog> PrettyExec<'a, PreLog, PostLog> {
    pub fn arg(&mut self, arg: &'a OsStr) -> &mut Self {
        self.arguments.push(arg);
        self.command.arg(arg);
        self
    }

    pub fn spawn(&'a mut self) -> io::Result<ExitStatus>
    where
        Logger<'a, PreLog, str, OsStr, Vec<&'a OsStr>>: Log,
        Logger<'a, PostLog, str, OsStr, Vec<&'a OsStr>>: Log,
    {
        Logger::new(&self.log_before, self.prompt, self.program, &self.arguments).log();
        let result = self.command.spawn()?.wait();
        Logger::new(&self.log_after, self.prompt, self.program, &self.arguments).log();
        result
    }

    pub fn set_log_before<Logger>(self, log_before: Logger) -> PrettyExec<'a, Logger, PostLog> {
        PrettyExec {
            prompt: self.prompt,
            program: self.program,
            arguments: self.arguments,
            command: self.command,
            log_after: self.log_after,
            log_before,
        }
    }

    pub fn set_log_after<Logger>(self, log_after: Logger) -> PrettyExec<'a, PreLog, Logger> {
        PrettyExec {
            prompt: self.prompt,
            program: self.program,
            arguments: self.arguments,
            command: self.command,
            log_before: self.log_before,
            log_after,
        }
    }
}

impl<'a> PrettyExec<'a, Nothing, Nothing> {
    pub fn new(prompt: &'a str, program: &'a OsStr) -> Self {
        PrettyExec {
            prompt,
            program,
            log_before: Nothing,
            log_after: Nothing,
            arguments: Vec::new(),
            command: Command::new(program),
        }
    }
}
