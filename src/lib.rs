//! SigmaOS-amavisd: Amavisd-new email scanner model.
//!
//! Real implementation: policies, scans, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub action: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scan {
    pub id: String,
    pub policy_id: String,
    pub result: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmavisdError {
    EmptyPolicyName,
    DuplicatePolicy(String),
}

impl fmt::Display for AmavisdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPolicyName => write!(f, "policy name must not be empty"),
            Self::DuplicatePolicy(id) => write!(f, "duplicate policy {:?}", id),
        }
    }
}

impl std::error::Error for AmavisdError {}

pub struct Amavisd {
    pub name: String,
    policies: BTreeMap<String, Policy>,
    scans: BTreeMap<String, Scan>,
}

impl Amavisd {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), policies: BTreeMap::new(), scans: BTreeMap::new() }
    }

    pub fn add_policy(&mut self, p: Policy) -> Result<(), AmavisdError> {
        if p.name.trim().is_empty() { return Err(AmavisdError::EmptyPolicyName); }
        let id = p.id.clone();
        if self.policies.contains_key(&id) {
            return Err(AmavisdError::DuplicatePolicy(id.clone()));
        }
        self.policies.insert(id, p);
        Ok(())
    }

    pub fn policy_count(&self) -> usize { self.policies.len() }

    pub fn get_policy(&self, id: &str) -> Option<&Policy> { self.policies.get(id) }

    pub fn add_scan(&mut self, s: Scan) {
        self.scans.insert(s.id.clone(), s);
    }

    pub fn get_scan(&self, id: &str) -> Option<&Scan> { self.scans.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = Amavisd::new("Amavisd");
        assert_eq!(a.name, "Amavisd");
        assert_eq!(a.policy_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = Amavisd::new("Amavisd");
        let policy = Policy { id: "p1".into(), name: "virus-scan".into(), action: "quarantine".into() };
        a.add_policy(policy).unwrap();
        assert_eq!(a.policy_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = Amavisd::new("Amavisd");
        let policy = Policy { id: "x".into(), name: "".into(), action: "x".into() };
        assert!(a.add_policy(policy).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = Amavisd::new("Amavisd");
        let policy = Policy { id: "x".into(), name: "X".into(), action: "x".into() };
        a.add_policy(policy).unwrap();
        let policy2 = Policy { id: "x".into(), name: "X".into(), action: "x".into() };
        assert!(a.add_policy(policy2).is_err());
    }
}
