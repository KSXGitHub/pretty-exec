use super::logger::{Logger, Nothing};
use std::{
    io,
    process::{Command, ExitStatus},
};

pub struct PrettyExec<PreLog, PostLog>
where
    PreLog: Logger,
    PostLog: Logger,
{
    pub program: String,
    pub arguments: Vec<String>,
    command: Command,
    log_before: PreLog,
    log_after: PostLog,
}

impl<PreLog, PostLog> PrettyExec<PreLog, PostLog>
where
    PreLog: Logger,
    PostLog: Logger,
{
    pub fn arg(&mut self, arg: &str) -> &mut Self {
        self.arguments.push(arg.to_owned());
        self.command.arg(arg);
        self
    }

    pub fn spawn(&mut self) -> io::Result<ExitStatus> {
        self.log_before.log(self.program.as_str(), &self.arguments);
        let result = self.command.spawn()?.wait();
        self.log_after.log(self.program.as_str(), &self.arguments);
        result
    }

    pub fn set_log_before<Logger: self::Logger>(
        self,
        log_before: Logger,
    ) -> PrettyExec<Logger, PostLog> {
        PrettyExec {
            program: self.program,
            arguments: self.arguments,
            command: self.command,
            log_after: self.log_after,
            log_before,
        }
    }

    pub fn set_log_after<Logger: self::Logger>(
        self,
        log_after: Logger,
    ) -> PrettyExec<PreLog, Logger> {
        PrettyExec {
            program: self.program,
            arguments: self.arguments,
            command: self.command,
            log_before: self.log_before,
            log_after,
        }
    }
}

impl PrettyExec<Nothing, Nothing> {
    pub fn new(program: &str) -> Self {
        PrettyExec {
            program: program.to_owned(),
            log_before: Nothing,
            log_after: Nothing,
            arguments: Vec::new(),
            command: Command::new(program),
        }
    }
}
