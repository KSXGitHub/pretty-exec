#![cfg(target_os = "linux")]
use run_shell::cmd;
use std::env::current_dir;

fn main() {
    println!("cargo:rerun-if-changed=assets/faketty.c");
    let assets = current_dir().expect("get current dir").join("assets");
    let source = assets.join("faketty.c").to_string_lossy().to_string();
    let output = assets.join("libfaketty.so").to_string_lossy().to_string();
    let command = format!("cc -shared -o {output} {source}");
    dbg!(&assets, &source, &output, &command);
    eprintln!("Compiling fake tty...");
    cmd!(&command).run().expect("compile fake-tty.so");
    println!("cargo:rustc-env=FAKE_TTY_DIR={}", assets.to_string_lossy());
    println!("cargo:rustc-env=FAKE_TTY_LIB={output}");
}
