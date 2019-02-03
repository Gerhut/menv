use std::env::current_dir;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn bin() -> PathBuf {
    let executable = if cfg!(windows) { "menv.exe" } else { "menv" };
    let mut path = PathBuf::from(current_dir().unwrap());
    path.push("target");
    path.push("debug");
    path.push(executable);
    path
}

fn fixture(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(current_dir().unwrap());
    path.push("tests");
    path.push("fixtures");
    path.push(filename);
    path
}

#[test]
fn file_to_file() {
    fs::write(fixture("1.in"), "{{{ FOO }}}").unwrap();

    let status = Command::new(bin())
        .current_dir(fixture("."))

        .arg("1.in")
        .arg("1.out")

        .env("FOO", "bar")

        .stdin(Stdio::null())
        .stdout(Stdio::null())

        .status()
        .unwrap();
    assert!(status.success());

    let actual = fs::read_to_string(fixture("1.out")).unwrap();
    assert_eq!(actual, "bar");
}

#[test]
fn file_to_console() {
    fs::write(fixture("2.in"), "{{{ FOO }}}").unwrap();

    let output = Command::new(bin())
        .current_dir(fixture("."))

        .arg("2.in")
        .arg("-")

        .env("FOO", "bar")

        .stdin(Stdio::null())
        .stdout(Stdio::piped())

        .output()
        .unwrap();
    assert!(output.status.success());

    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, "bar");
}

#[test]
fn console_to_file() {
    let mut child = Command::new(bin())
        .current_dir(fixture("."))

        .arg("-")
        .arg("3.out")

        .env("FOO", "bar")

        .stdin(Stdio::piped())
        .stdout(Stdio::null())

        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap()
        .write_all("{{{ FOO }}}".as_bytes()).unwrap();
    assert!(child.wait().unwrap().success());

    let actual = fs::read_to_string(fixture("3.out")).unwrap();
    assert_eq!(actual, "bar");
}

#[test]
fn console_to_console() {
    let mut child = Command::new(bin())
        .current_dir(fixture("."))

        .arg("-")
        .arg("-")

        .env("FOO", "bar")

        .stdin(Stdio::piped())
        .stdout(Stdio::piped())

        .spawn()
        .unwrap();
    child.stdin.as_mut().unwrap()
        .write_all("{{{ FOO }}}".as_bytes()).unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, "bar");
}

#[test]
fn with_dotenv() {
    fs::write(fixture(".env"), "FOO=bar").unwrap();
    let mut child = Command::new(bin())
        .current_dir(fixture("."))

        .arg("-d")
        .arg("-")
        .arg("-")

        .stdin(Stdio::piped())
        .stdout(Stdio::piped())

        .spawn()
        .unwrap();
    child.stdin.as_mut().unwrap()
        .write_all("{{{ FOO }}}".as_bytes()).unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, "bar");
}

#[test]
fn render_failed() {
    let mut child = Command::new(bin())
        .current_dir(fixture("."))

        .arg("-d")
        .arg("-")
        .arg("-")

        .stdin(Stdio::piped())
        .stdout(Stdio::piped())

        .spawn()
        .unwrap();
    child.stdin.as_mut().unwrap()
        .write_all("{{{".as_bytes()).unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(!output.status.success());
}
