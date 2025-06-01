use assert_cmd::Command;
use expect_test::expect_file;
use std::{path::PathBuf, sync::LazyLock};

static LD_LIBRARY_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let output = Command::new("rustc").arg("--print=sysroot").output().unwrap();
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr).unwrap();
        panic!("Failed to run `rustc --print=sysroot`:\nstdout={stdout}\nstderr={stderr}")
    }
    let path = PathBuf::from(stdout.trim()).join("lib");
    assert!(path.exists());
    path
});

fn compile(file: &str) -> (&'static str, std::process::Output) {
    let exe = env!("CARGO_PKG_NAME");
    let output = Command::cargo_bin(exe)
        .unwrap()
        .arg(file)
        .arg("--crate-type=lib")
        // .arg("--edition=2024")
        .env("STOP_COMPILATION", "1")
        .env("LD_LIBRARY_PATH", &*LD_LIBRARY_PATH)
        .output()
        .unwrap();
    (exe, output)
}

fn should_panic(file: &str, outfile: &str) {
    let (exe, output) = compile(file);
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    if output.status.success() {
        panic!("`{exe} {file}` should panic:\nstdout={stdout}\nstderr={stderr}")
    }
    let out = format!("stdout=\n{stdout}\nstderr=\n{stderr}");
    expect_file![outfile].assert_eq(&out);
}

fn testcase(name: &str) -> [String; 2] {
    let file = format!("./tests/snippets/{name}.rs");
    let outfile = format!("snapshots/{name}.txt");
    [file, outfile]
}

#[test]
fn unsafe_calls() {
    let [file, outfile] = &testcase("unsafe_calls");
    let (exe, output) = compile(file);
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    if !output.status.success() {
        panic!("Failed to run `{exe} {file}`:\nstdout={stdout}\nstderr={stderr}")
    }
    let out = format!("stdout=\n{stdout}\nstderr=\n{stderr}");
    expect_file![outfile].assert_eq(&out);
}

#[test]
fn unsafe_calls_panic_assign() {
    let [file, outfile] = &testcase("unsafe_calls_panic_assign");
    should_panic(file, outfile);
}

#[test]
fn unsafe_calls_panic_assign_fn_ptr() {
    let [file, outfile] = &testcase("unsafe_calls_panic_assign_fn_ptr");
    should_panic(file, outfile);
}

#[test]
fn unsafe_calls_panic_no_tag() {
    let [file, outfile] = &testcase("unsafe_calls_panic_no_tag");
    should_panic(file, outfile);
}
