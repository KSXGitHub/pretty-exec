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

fn get_json_version() -> String {
    include_str!("../../../sync.json")
        .pipe(serde_json::from_str::<SyncJson>)
        .pipe(Result::unwrap)
        .pipe(|x: SyncJson| x.version)
}

fn test_against(toml_text: &str) {
    let json_version = get_json_version();
    let toml_version = toml_text
        .pipe(toml::from_str::<CargoManifest>)
        .pipe(Result::unwrap)
        .pipe(|x: CargoManifest| x.package.version);
    assert_eq!(toml_version, json_version);
}

#[test]
fn bin_version() {
    test_against(include_str!("../Cargo.toml"));
}

#[test]
fn lib_version() {
    test_against(include_str!("../../pretty-exec-lib/Cargo.toml"));
}
