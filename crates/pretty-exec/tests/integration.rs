#![cfg(test)]
use std::process::Command;
use yansi::{Color, Style};

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
fn missing_program() {
    let output = exe().output().unwrap();

    let expected_stdout = String::new();
    let expected_stderr = "ERROR: Program is not specified\n".to_string();
    let actual_stderr = u8v_to_utf8(&output.stderr);
    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(
        (actual_stderr, actual_stdout),
        (expected_stderr, expected_stdout),
    );
}

#[cfg(unix)]
#[test]
fn program_not_exist() {
    let output = exe().arg("program that does not exist").output().unwrap();

    let expected_stdout = String::new();
    let expected_stderr = "$ 'program that does not exist'\nERROR: Failed to spawn subprocess: No such file or directory (os error 2)\n".to_string();
    let actual_stderr = u8v_to_utf8(&output.stderr);
    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(
        (actual_stderr, actual_stdout),
        (expected_stderr, expected_stdout),
    );
}

#[test]
fn color_always() {
    let output = exe()
        .arg("--color=always")
        .arg("--")
        .arg("echo")
        .arg("hello")
        .arg("--world")
        .arg("-abc")
        .arg("--abc=def")
        .output()
        .unwrap();

    let expected_stdout = "hello --world -abc --abc=def\n".to_string();
    let expected_stderr = format!("{}\n", expected_colorful_title());
    let actual_stderr = u8v_to_utf8(&output.stderr);
    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(
        (actual_stderr, actual_stdout),
        (expected_stderr, expected_stdout),
    );
}

#[test]
fn color_never() {
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

    let expected_stderr = "$ echo hello --world -abc --abc=def\n".to_string();
    let expected_stdout = "hello --world -abc --abc=def\n".to_string();
    let actual_stderr = u8v_to_utf8(&output.stderr);
    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(
        (actual_stderr, actual_stdout),
        (expected_stderr, expected_stdout),
    );
}

#[test]
fn different_prompt() {
    let output = exe()
        .arg("--prompt=>>>")
        .arg("--")
        .arg("echo")
        .arg("hello")
        .arg("--world")
        .arg("-abc")
        .arg("--abc=def")
        .output()
        .unwrap();

    let expected_stderr = ">>> echo hello --world -abc --abc=def\n".to_string();
    let expected_stdout = "hello --world -abc --abc=def\n".to_string();
    let actual_stderr = u8v_to_utf8(&output.stderr);
    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(
        (actual_stderr, actual_stdout),
        (expected_stderr, expected_stdout),
    );
}

#[test]
fn empty_prompt() {
    let output = exe()
        .arg("--prompt=")
        .arg("--")
        .arg("echo")
        .arg("hello")
        .arg("--world")
        .arg("-abc")
        .arg("--abc=def")
        .output()
        .unwrap();

    let expected_stderr = "echo hello --world -abc --abc=def\n".to_string();
    let expected_stdout = "hello --world -abc --abc=def\n".to_string();
    let actual_stderr = u8v_to_utf8(&output.stderr);
    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(
        (actual_stderr, actual_stdout),
        (expected_stderr, expected_stdout),
    );
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
        cmd = format_args!("::group::{}", expected_colorful_title()),
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

    let expected_stderr = "$ echo hello --world -abc --abc=def\n".to_string();
    let expected_stdout = "".to_string();
    let actual_stderr = u8v_to_utf8(&output.stderr);
    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(
        (actual_stderr, actual_stdout),
        (expected_stderr, expected_stdout),
    );
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

    let expected_stderr = "$ echo 'abc def ghi' 'jkl mno' pqrs '>' '>>'\n".to_string();
    let expected_stdout = "abc def ghi jkl mno pqrs > >>\n".to_string();
    let actual_stderr = u8v_to_utf8(&output.stderr);
    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(
        (actual_stderr, actual_stdout),
        (expected_stderr, expected_stdout),
    );
}

#[test]
fn escape_flags_with_whitespaces() {
    let output = exe()
        .arg("--color=never")
        .arg("--")
        .arg("echo")
        .arg("--abc=def")
        .arg("--abc=d e f")
        .arg("--a b c=def")
        .arg("--a b c = d e f")
        .arg("--a b c")
        .output()
        .unwrap();

    let expected_stderr =
        "$ echo --abc=def --abc='d e f' '--a b c'=def '--a b c '=' d e f' '--a b c'\n".to_string();
    let expected_stdout = "--abc=def --abc=d e f --a b c=def --a b c = d e f --a b c\n".to_string();
    let actual_stderr = u8v_to_utf8(&output.stderr);
    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(
        (actual_stderr, actual_stdout),
        (expected_stderr, expected_stdout),
    );
}

#[test]
fn escape_flags_with_whitespaces_colorful() {
    let output = exe()
        .arg("--color=always")
        .arg("--")
        .arg("echo")
        .arg("--abc=def")
        .arg("--abc=d e f")
        .arg("--a b c=def")
        .arg("--a b c = d e f")
        .arg("--a b c")
        .output()
        .unwrap();

    let expected_stderr = format!(
        "{} {} {}=def {}='d e f' {}=def {}=' d e f' {}\n",
        Style::default().dimmed().paint("$"),
        Color::Green.paint("echo"),
        Color::Red.paint("--abc"),
        Color::Red.paint("--abc"),
        Color::Red.paint("'--a b c'"),
        Color::Red.paint("'--a b c '"),
        Color::Red.paint("'--a b c'"),
    );
    let expected_stdout = "--abc=def --abc=d e f --a b c=def --a b c = d e f --a b c\n".to_string();
    let actual_stderr = u8v_to_utf8(&output.stderr);
    let actual_stdout = u8v_to_utf8(&output.stdout);

    assert_eq!(
        (actual_stderr, actual_stdout),
        (expected_stderr, expected_stdout),
    );
}
