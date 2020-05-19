use super::logger::{Logger, Nothing};
use std::{
    ffi::OsStr,
    io,
    process::{Child, Command},
};

pub struct PrettyExec<Program, Argument, PreLog, PostLog>
where
    Program: AsRef<OsStr>,
    Argument: AsRef<OsStr>,
    PreLog: Logger<Program, Argument>,
    PostLog: Logger<Program, Argument>,
{
    pub program: Program,
    pub arguments: Vec<Argument>,
    command: Command,
    log_before: PreLog,
    log_after: PostLog,
}

impl<Program, Argument, PreLog, PostLog> PrettyExec<Program, Argument, PreLog, PostLog>
where
    Program: AsRef<OsStr> + Copy,
    Argument: AsRef<OsStr> + Copy,
    PreLog: Logger<Program, Argument>,
    PostLog: Logger<Program, Argument>,
{
    pub fn arg(&mut self, arg: Argument) -> &mut Self {
        self.arguments.push(arg);
        self.command.arg(arg);
        self
    }

    pub fn spawn(&mut self) -> io::Result<Child> {
        self.log_before.log(self.program, &self.arguments);
        let result = self.command.spawn();
        self.log_after.log(self.program, &self.arguments);
        result
    }

    pub fn set_log_before<Logger: self::Logger<Program, Argument>>(
        self,
        log_before: Logger,
    ) -> PrettyExec<Program, Argument, Logger, PostLog> {
        PrettyExec {
            program: self.program,
            arguments: self.arguments,
            command: self.command,
            log_after: self.log_after,
            log_before,
        }
    }

    pub fn set_log_after<Logger: self::Logger<Program, Argument>>(
        self,
        log_after: Logger,
    ) -> PrettyExec<Program, Argument, PreLog, Logger> {
        PrettyExec {
            program: self.program,
            arguments: self.arguments,
            command: self.command,
            log_before: self.log_before,
            log_after,
        }
    }
}

impl<Program, Argument> PrettyExec<Program, Argument, Nothing, Nothing>
where
    Program: AsRef<OsStr> + Copy,
    Argument: AsRef<OsStr>,
{
    pub fn new(program: Program) -> Self {
        PrettyExec {
            program,
            log_before: Nothing,
            log_after: Nothing,
            arguments: Vec::new(),
            command: Command::new(program),
        }
    }
}
