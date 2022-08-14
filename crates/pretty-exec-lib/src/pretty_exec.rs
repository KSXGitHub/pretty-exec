use super::log::{Log, Logger, Nothing};
use std::{
    io,
    process::{Command, ExitStatus},
};

pub struct PrettyExec<PreLog, PostLog> {
    prompt: String,
    program: String,
    arguments: Vec<String>,
    command: Command,
    log_before: PreLog,
    log_after: PostLog,
}

impl<PreLog, PostLog> PrettyExec<PreLog, PostLog> {
    pub fn arg(&mut self, arg: &str) -> &mut Self {
        self.arguments.push(arg.to_owned());
        self.command.arg(arg);
        self
    }

    pub fn spawn<'a>(&'a mut self) -> io::Result<ExitStatus>
    where
        Logger<'a, PreLog, String, String, Vec<String>>: Log,
        Logger<'a, PostLog, String, String, Vec<String>>: Log,
    {
        Logger::new(
            &self.log_before,
            &self.prompt,
            &self.program,
            &self.arguments,
        )
        .log();
        let result = self.command.spawn()?.wait();
        Logger::new(
            &self.log_after,
            &self.prompt,
            &self.program,
            &self.arguments,
        )
        .log();
        result
    }

    pub fn set_log_before<Logger>(self, log_before: Logger) -> PrettyExec<Logger, PostLog> {
        PrettyExec {
            prompt: self.prompt,
            program: self.program,
            arguments: self.arguments,
            command: self.command,
            log_after: self.log_after,
            log_before,
        }
    }

    pub fn set_log_after<Logger>(self, log_after: Logger) -> PrettyExec<PreLog, Logger> {
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

impl PrettyExec<Nothing, Nothing> {
    pub fn new(prompt: &str, program: &str) -> Self {
        PrettyExec {
            prompt: prompt.to_owned(),
            program: program.to_owned(),
            log_before: Nothing,
            log_after: Nothing,
            arguments: Vec::new(),
            command: Command::new(program),
        }
    }
}
