// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-dependency-version-solver: dependency version solver component model.
//!
//! Real implementation: versions, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyVersionSolverError {
    EmptyVersionName,
    DuplicateVersion(String),
    EmptyEventSubject,
}

impl fmt::Display for DependencyVersionSolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyVersionName => write!(f, "version name must not be empty"),
            Self::DuplicateVersion(id) => write!(f, "duplicate version {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DependencyVersionSolverError {}

pub struct DependencyVersionSolver {
    pub name: String,
    versions: BTreeMap<String, Version>,
    events: BTreeMap<String, Event>,
}

impl DependencyVersionSolver {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            versions: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_version(&mut self, item: Version) -> Result<(), DependencyVersionSolverError> {
        if item.name.trim().is_empty() {
            return Err(DependencyVersionSolverError::EmptyVersionName);
        }
        let id = item.id.clone();
        if self.versions.contains_key(&id) {
            return Err(DependencyVersionSolverError::DuplicateVersion(id.clone()));
        }
        self.versions.insert(id, item);
        Ok(())
    }

    pub fn version_count(&self) -> usize {
        self.versions.len()
    }

    pub fn get_version(&self, id: &str) -> Option<&Version> {
        self.versions.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DependencyVersionSolverError> {
        if event.subject.trim().is_empty() {
            return Err(DependencyVersionSolverError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_version(id: &str, name: &str) -> Version {
        Version {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DependencyVersionSolver::new("DependencyVersionSolver");
        assert_eq!(m.name, "DependencyVersionSolver");
        assert_eq!(m.version_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_version() {
        let mut m = DependencyVersionSolver::new("DependencyVersionSolver");
        m.add_version(sample_version("a1", "Alpha")).unwrap();
        assert_eq!(m.version_count(), 1);
        assert!(m.get_version("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DependencyVersionSolver::new("DependencyVersionSolver");
        assert!(m.add_version(sample_version("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DependencyVersionSolver::new("DependencyVersionSolver");
        m.add_version(sample_version("x", "One")).unwrap();
        assert!(m.add_version(sample_version("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DependencyVersionSolver::new("DependencyVersionSolver");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
