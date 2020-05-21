#![cfg(test)]
use ansi_term::*;
use std::process::Command;

const EXE: &str = env!("CARGO_BIN_EXE_pretty-exec");

fn exe() -> Command {
    Command::new(EXE)
}

fn u8v_to_utf8(u8v: &[u8]) -> String {
    std::str::from_utf8(u8v)
        .expect("convert a vector of bytes to UTF-8 string")
        .replace("\r\n", "\n")
}

fn expected_colorful_title() -> String {
    format!(
        "{} {} hello {} {} {}=def",
        Style::default().dimmed().paint("$"),
        Color::Green.paint("echo"),
        Color::Red.paint("--world"),
        Color::Red.paint("-abc"),
        Color::Red.paint("--abc"),
    )
}

#[test]
fn default_stdout() {
    let output = exe()
        .arg("--")
        .arg("echo")
        .arg("hello")
        .arg("--world")
        .arg("-abc")
        .arg("--abc=def")
        .output()
        .unwrap();

    let expected_stdout = format!(
        "{cmd}\n{output}\n",
        cmd = expected_colorful_title(),
        output = "hello --world -abc --abc=def",
    );

    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(actual_stdout, expected_stdout);
}

#[test]
fn color_never_stdout() {
    let output = exe()
        .arg("--color=never")
        .arg("--")
        .arg("echo")
        .arg("hello")
        .arg("--world")
        .arg("-abc")
        .arg("--abc=def")
        .output()
        .unwrap();

    let expected_stdout = format!(
        "{cmd}\n{output}\n",
        cmd = "$ echo hello --world -abc --abc=def",
        output = "hello --world -abc --abc=def",
    );

    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(actual_stdout, expected_stdout);
}

#[test]
fn github_actions() {
    let output = exe()
        .arg("--github-actions")
        .arg("--")
        .arg("echo")
        .arg("hello")
        .arg("--world")
        .arg("-abc")
        .arg("--abc=def")
        .output()
        .unwrap();

    let expected_stdout = format!(
        "{cmd}\n{output}\n{endgroup}\n",
        cmd = format!("::group::{}", expected_colorful_title()),
        output = "hello --world -abc --abc=def",
        endgroup = "::endgroup::",
    );

    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(actual_stdout, expected_stdout);
}

#[test]
fn github_actions_color_never() {
    let output = exe()
        .arg("--github-actions")
        .arg("--color=never")
        .arg("--")
        .arg("echo")
        .arg("hello")
        .arg("--world")
        .arg("-abc")
        .arg("--abc=def")
        .output()
        .unwrap();

    let expected_stdout = format!(
        "{cmd}\n{output}\n{endgroup}\n",
        cmd = "::group::$ echo hello --world -abc --abc=def",
        output = "hello --world -abc --abc=def",
        endgroup = "::endgroup::",
    );

    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(actual_stdout, expected_stdout);
}

#[test]
fn skip_exec_stdout() {
    let output = exe()
        .arg("--skip-exec")
        .arg("--")
        .arg("echo")
        .arg("hello")
        .arg("--world")
        .arg("-abc")
        .arg("--abc=def")
        .output()
        .unwrap();

    let expected_stdout = format!("{}\n", expected_colorful_title());
    let actual_stdout = u8v_to_utf8(&output.stdout);
    assert_eq!(actual_stdout, expected_stdout);
}

#[test]
fn default_status() {
    let output = exe()
        .arg("--")
        .arg("sh")
        .arg("-c")
        .arg("exit 123")
        .output()
        .unwrap();

    let status = output.status.code().unwrap();
    assert_eq!(status, 123);
}

#[test]
fn shell_escape() {
    let output = exe()
        .arg("--color=never")
        .arg("echo")
        .arg("abc def ghi")
        .arg("jkl mno")
        .arg("pqrs")
        .arg(">")
        .arg(">>")
        .output()
        .unwrap();

    let expected_stdout = format!(
        "{cmd}\n{output}\n",
        cmd = "$ echo 'abc def ghi' 'jkl mno' pqrs '>' '>>'",
        output = "abc def ghi jkl mno pqrs > >>",
    );

    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(actual_stdout, expected_stdout);
}
