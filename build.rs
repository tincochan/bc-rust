use std::process::Command;

fn main() {
    let oup = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    let commit_hash = String::from_utf8(oup.stdout).unwrap();
    println!("cargo:rustc-env=COMMIT_HASH={}", commit_hash);
}
