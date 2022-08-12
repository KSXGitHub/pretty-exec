use super::super::app::args::Args;
use clap_utilities::CommandFactoryExtra;
use std::process::ExitCode;

pub fn main() -> ExitCode {
    Args::run_completion_generator()
}
