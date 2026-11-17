//! Integration tests for SigmaOS-dependency-version-solver.

use sigmaos_dependency_version_solver_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
