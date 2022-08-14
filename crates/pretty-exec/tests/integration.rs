#![cfg(test)]
use nu_ansi_term::{Color, Style};
use pty_process::Command as _;
use std::{
    io::Read,
    process::{Command, Stdio},
};

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

macro_rules! read_child_stdio {
    ($child:expr, $field:ident) => {{
        let mut output = String::new();
        $child
            .$field
            .as_mut()
            .expect(concat!("get ", stringify!($field)))
            .read_to_string(&mut output)
            .expect(concat!("read ", stringify!($field)));
        output
    }};
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
fn color_auto_piped() {
    let output = exe()
        .arg("--color=auto")
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
fn color_auto_pty() {
    let mut child = exe()
        .arg("--color=auto")
        .arg("--")
        .arg("echo")
        .arg("hello")
        .arg("--world")
        .arg("-abc")
        .arg("--abc=def")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn_pty(None)
        .expect("spawn process in a pty");

    let expected_stdout = "hello --world -abc --abc=def\n".to_string();
    let expected_stderr = format!("{}\n", expected_colorful_title());
    let actual_stdout = read_child_stdio!(child, stdout);
    let actual_stderr = read_child_stdio!(child, stderr);

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
