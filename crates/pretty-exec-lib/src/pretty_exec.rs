use super::log::{Log, Logger, Nothing};
use pipe_trait::Pipe;
use std::{
    ffi::OsStr,
    io,
    ops::Deref,
    process::{Command, ExitStatus},
};

pub struct PrettyExec<Prompt, Program, Arguments, PreLog, PostLog> {
    prompt: Prompt,
    program: Program,
    arguments: Arguments,
    log_before: PreLog,
    log_after: PostLog,
}

impl<Prompt, Program, Arguments, PreLog, PostLog>
    PrettyExec<Prompt, Program, Arguments, PreLog, PostLog>
{
    pub fn spawn<'a>(&'a mut self) -> io::Result<ExitStatus>
    where
        Prompt: Deref,
        Program: AsRef<OsStr> + Deref,
        Arguments: Deref,
        &'a Arguments::Target: IntoIterator,
        <&'a Arguments::Target as IntoIterator>::Item: AsRef<OsStr>,
        Logger<'a, PreLog, Prompt::Target, Program::Target, Arguments::Target>: Log,
        Logger<'a, PostLog, Prompt::Target, Program::Target, Arguments::Target>: Log,
    {
        Logger::new(
            &self.log_before,
            self.prompt.deref(),
            self.program.deref(),
            self.arguments.deref(),
        )
        .log();
        let result = self
            .program
            .pipe_ref(Command::new)
            .args(self.arguments.deref())
            .spawn()?
            .wait();
        Logger::new(
            &self.log_after,
            self.prompt.deref(),
            self.program.deref(),
            self.arguments.deref(),
        )
        .log();
        result
    }

    pub fn set_log_before<Logger>(
        self,
        log_before: Logger,
    ) -> PrettyExec<Prompt, Program, Arguments, Logger, PostLog> {
        PrettyExec {
            prompt: self.prompt,
            program: self.program,
            arguments: self.arguments,
            log_after: self.log_after,
            log_before,
        }
    }

    pub fn set_log_after<Logger>(
        self,
        log_after: Logger,
    ) -> PrettyExec<Prompt, Program, Arguments, PreLog, Logger> {
        PrettyExec {
            prompt: self.prompt,
            program: self.program,
            arguments: self.arguments,
            log_before: self.log_before,
            log_after,
        }
    }
}

impl<Prompt, Program, Arguments> PrettyExec<Prompt, Program, Arguments, Nothing, Nothing> {
    pub fn new(prompt: Prompt, program: Program, arguments: Arguments) -> Self {
        PrettyExec {
            prompt,
            program,
            arguments,
            log_before: Nothing,
            log_after: Nothing,
        }
    }
}
