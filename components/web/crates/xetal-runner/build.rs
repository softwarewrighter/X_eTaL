//! A build id for the worker's files, `XETAL_RUNNER_BUILD`: the page
//! loads the worker through URLs carrying it, so a browser (or GitHub
//! Pages, which caches for ten minutes) never runs a new page with an
//! old worker. The worker's files are not content-hashed by trunk, so
//! the id is the build's time; it changes whenever this crate, the
//! standard libraries it embeds or any component is built again.

fn main() {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    println!("cargo:rustc-env=XETAL_RUNNER_BUILD={secs}");
    for dir in ["../../../../lib", "../../../../components"] {
        println!("cargo:rerun-if-changed={dir}");
    }
}
