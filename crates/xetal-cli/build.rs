//! Emits `BUILD_HOST`, `GIT_HASH` and `BUILD_TIMESTAMP` for the
//! `--version` block, following the Software Wrighter CLI convention.

use std::process::Command;

fn main() {
    let host = hostname::get().map_or_else(
        |_| "unknown".to_string(),
        |h| h.to_string_lossy().to_string(),
    );
    println!("cargo:rustc-env=BUILD_HOST={host}");

    let hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map_or_else(
            || "unknown".to_string(),
            |o| String::from_utf8_lossy(&o.stdout).trim().to_string(),
        );
    println!("cargo:rustc-env=GIT_HASH={hash}");

    let timestamp = chrono::Local::now()
        .format("%Y-%m-%dT%H:%M:%S%z")
        .to_string();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={timestamp}");

    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/refs/heads");
    println!("cargo:rerun-if-changed=build.rs");
}
