use clap::ValueEnum;

#[derive(Eq, PartialEq, Debug, Copy, Clone, ValueEnum)]
pub enum When {
    Auto,
    Never,
    Always,
}
