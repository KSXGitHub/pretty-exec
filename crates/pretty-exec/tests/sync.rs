use pipe_trait::*;
use pretty_exec::{args::Args, clap_complete::Shell, clap_utilities::CommandFactoryExtra};
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
    ($name:ident: $shell:ident -> $path:literal) => {
        #[test]
        fn $name() {
            eprintln!(
                "check!({name}: {shell} -> {path});",
                name = stringify!($name),
                shell = stringify!($shell),
                path = $path,
            );
            let received = Args::get_completion_string("pretty-exec", Shell::$shell)
                .expect("get completion string");
            let expected = include_str!($path);
            assert!(received == expected, "completion is outdated");
        }
    };
}

test_completion!(bash_completion: Bash -> "../../../exports/completion.bash");
test_completion!(fish_completion: Fish -> "../../../exports/completion.fish");
test_completion!(zsh_completion: Zsh -> "../../../exports/completion.zsh");
test_completion!(powershell_completion: PowerShell -> "../../../exports/completion.ps1");
test_completion!(elvish_completion: Elvish -> "../../../exports/completion.elv");
