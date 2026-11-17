// Integration test for SigmaOS-dependency-version-solver
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use dependency_version_solver as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
