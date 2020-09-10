use std::path::PathBuf;
use structopt::*;

pub use clap::Shell;

#[derive(StructOpt)]
#[structopt(name = "pretty-exec-completions", rename_all = "kebab")]
pub struct Args {
    /// Binary name
    #[structopt(long, default_value = "pretty-exec")]
    pub bin: String,

    /// File to write to
    /// [default: stdout]
    #[structopt(long, short = "o")]
    pub output: Option<PathBuf>,

    /// Type of shell
    #[structopt(name = "shell", possible_values = &["bash", "fish", "zsh", "powershell", "elvish"])]
    pub shell: Shell,
}

impl Args {
    pub fn get(args: impl Iterator<Item = String>) -> Self {
        use std::process::exit;
        match Self::from_iter_safe(args) as Result<Self, clap::Error> {
            Ok(value) => value,
            Err(clap::Error { kind, message, .. }) => match kind {
                clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed => {
                    println!("{}", message);
                    exit(0);
                }
                _ => {
                    println!("{}", message);
                    exit(1);
                }
            },
        }
    }
}
