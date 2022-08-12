use super::super::app::args::Args;
use structopt_utilities::StructOptUtils;

pub fn main() {
    Args::run_completion_generator("pretty-exec-completions", "pretty-exec")
}
