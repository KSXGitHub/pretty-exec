use pipe_trait::*;
use pretty_exec::{args::Args, clap::Shell, structopt::StructOpt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CargoManifest {
    pub package: CargoPackage,
}

#[derive(Debug, Serialize, Deserialize)]
struct CargoPackage {
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SyncJson {
    pub version: String,
}

macro_rules! test_version {
    ($name:ident, $path:literal) => {
        #[test]
        fn $name() {
            let json_version = include_str!("../../../sync.json")
                .pipe(serde_json::from_str::<SyncJson>)
                .pipe(Result::unwrap)
                .pipe(|x: SyncJson| x.version);
            let toml_version = include_str!($path)
                .pipe(toml::from_str::<CargoManifest>)
                .pipe(Result::unwrap)
                .pipe(|x: CargoManifest| x.package.version);
            assert_eq!(toml_version, json_version);
        }
    };
}

test_version!(bin_version, "../Cargo.toml");
test_version!(lib_version, "../../pretty-exec-lib/Cargo.toml");

macro_rules! test_completion {
    ($test_name:ident, $shell:expr, $path:literal) => {
        #[test]
        fn $test_name() {
            let expected: &[u8] = include_bytes!($path);
            let mut actual = Vec::new();
            Args::clap().gen_completions_to("pretty-exec", $shell, &mut actual);
            let actual = actual.as_slice();
            assert_eq!(actual, expected);
        }
    };
}

test_completion!(
    bash_completion,
    Shell::Bash,
    "../../../exports/completion.bash"
);
test_completion!(
    fish_completion,
    Shell::Fish,
    "../../../exports/completion.fish"
);
test_completion!(
    zsh_completion,
    Shell::Zsh,
    "../../../exports/completion.zsh"
);
test_completion!(
    powershell_completion,
    Shell::PowerShell,
    "../../../exports/completion.ps1"
);
test_completion!(
    elvish_completion,
    Shell::Elvish,
    "../../../exports/completion.elv"
);
