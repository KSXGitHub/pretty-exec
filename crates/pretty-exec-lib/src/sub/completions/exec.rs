use super::super::super::main::args::Args as MainArgs;
use super::args::Args;
use pipe_trait::*;
use std::{
    fs::File,
    io::{stdout, Write},
};
use structopt::StructOpt;

pub fn exec(args: Args) {
    let Args { bin, output, shell } = args;

    let mut buf: Box<dyn Write> = if let Some(file_name) = output {
        file_name.pipe(File::create).unwrap().pipe(Box::new)
    } else {
        Box::new(stdout())
    };

    MainArgs::clap().gen_completions_to(bin, shell, &mut buf);
}
