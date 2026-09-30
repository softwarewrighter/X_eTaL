//! Build provenance for the live demo's footer (as the other live
//! demos show it): the build host, the short commit and the build time.

use std::process::Command;

fn main() {
    println!(
        "cargo:rustc-env=BUILD_HOST={}",
        capture("hostname", &["-s"])
    );
    let sha = capture("git", &["rev-parse", "--short", "HEAD"]);
    println!("cargo:rustc-env=BUILD_SHA={sha}");
    let time = capture("date", &["-u", "+%Y-%m-%dT%H:%M:%SZ"]);
    println!("cargo:rustc-env=BUILD_TIMESTAMP={time}");
    // Watch the git ref too, or the commit would stay at the first build's.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../../../.git/refs/heads");
}

fn capture(program: &str, args: &[&str]) -> String {
    Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".into())
}
