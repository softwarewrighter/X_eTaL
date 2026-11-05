//! The live demo's worker (built by trunk as a Web Worker).

fn main() {
    // The worker runs the programs: it finds the collection's libraries too.
    xetal_collection::install();
    xetal_runner::start_worker();
}
