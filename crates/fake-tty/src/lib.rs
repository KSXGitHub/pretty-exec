#![cfg(target_os = "linux")]
use std::{env::var, process::Command};

pub const FAKE_TTY_DIR: &str = env!("FAKE_TTY_DIR");
pub const FAKE_TTY_LIB: &str = env!("FAKE_TTY_LIB");

pub trait FakeTty {
    fn set_fake_tty(&mut self) -> &mut Self;

    fn with_fake_tty(mut self) -> Self
    where
        Self: Sized,
    {
        self.set_fake_tty();
        self
    }
}

impl FakeTty for Command {
    fn set_fake_tty(&mut self) -> &mut Self {
        let ld_preload = match var("LD_PRELOAD") {
            Ok(ld_preload) => format!("{FAKE_TTY_LIB}:{ld_preload}"),
            Err(_) => FAKE_TTY_LIB.to_string(),
        };

        self.env("LD_PRELOAD", ld_preload);
        self
    }
}
