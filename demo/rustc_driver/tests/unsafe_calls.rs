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

#[test]
fn unsafe_calls() {
    let exe = env!("CARGO_PKG_NAME");
    let file = "./tests/snippets/unsafe_calls.rs";
    let output = Command::cargo_bin(exe)
        .unwrap()
        .arg(file)
        .arg("--crate-type=lib")
        // .arg("--edition=2024")
        .env("STOP_COMPILATION", "1")
        .env("LD_LIBRARY_PATH", &*LD_LIBRARY_PATH)
        .output()
        .unwrap();
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    if !output.status.success() {
        panic!("Failed to run `{exe} {file}`:\nstdout={stdout}\nstderr={stderr}")
    }
    let out = format!("stdout=\n{stdout}\nstderr=\n{stderr}");
    expect_file!["snapshots/unsafe_calls.txt"].assert_eq(&out);
}
