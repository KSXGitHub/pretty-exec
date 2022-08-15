use clap::ValueEnum;

#[derive(Eq, PartialEq, Debug, Copy, Clone, ValueEnum)]
#[cfg_attr(test, derive(Ord, PartialOrd))]
pub enum When {
    Auto,
    Never,
    Always,
}
