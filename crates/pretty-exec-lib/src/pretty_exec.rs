use super::logger::Logger;
use std::{
    ffi::OsStr,
    io,
    process::{Child, Command},
};

pub struct PrettyExec<Program, Argument, PreLog, PostLog>
where
    Program: AsRef<OsStr>,
    Argument: AsRef<OsStr>,
    PreLog: Logger,
    PostLog: Logger,
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
    PreLog: Logger,
    PostLog: Logger,
{
    pub fn new(program: Program, log_before: PreLog, log_after: PostLog) -> Self {
        PrettyExec {
            program,
            log_before,
            log_after,
            arguments: Vec::new(),
            command: Command::new(program),
        }
    }

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
}
