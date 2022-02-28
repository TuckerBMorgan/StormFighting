// build.rs
use std::process::Command;
use std::io::{self, Write};

fn main() {
    return;
    let output = Command::new("./web.bat").output().expect("web build failed");
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());
}