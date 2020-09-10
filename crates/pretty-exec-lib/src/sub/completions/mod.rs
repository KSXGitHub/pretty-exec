pub mod args;
pub mod exec;

use pipe_trait::*;
use std::env;

pub fn main() {
    env::args().pipe(args::Args::get).pipe(exec::exec)
}
