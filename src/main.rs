//! SigmaOS-dependency-version-solver — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_dependency_version_solver_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-dependency-version-solver: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-dependency-version-solver: ok");
}
