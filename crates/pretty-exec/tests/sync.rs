use pipe_trait::*;
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
