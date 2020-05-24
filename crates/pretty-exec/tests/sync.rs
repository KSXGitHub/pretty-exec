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

fn get_version(text: &str) -> String {
    text.pipe(toml::from_str::<CargoManifest>)
        .pipe(Result::unwrap)
        .pipe(|x: CargoManifest| x.package.version)
}

#[test]
fn version() {
    let bin_version = get_version(include_str!("../Cargo.toml"));
    let lib_version = get_version(include_str!("../../pretty-exec-lib/Cargo.toml"));
    assert_eq!(bin_version, lib_version);
}
