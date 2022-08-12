#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum When {
    Auto,
    Never,
    Always,
}

impl std::str::FromStr for When {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(match text {
            "auto" => When::Auto,
            "never" => When::Never,
            "always" => When::Always,
            _ => return Err(text.to_owned()),
        })
    }
}
