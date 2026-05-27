//! Agent capabilities as category-theoretic objects.

use std::collections::HashMap;
use std::fmt;

/// A capability that an agent possesses — an object in the capability category.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Capability {
    /// Unique name of the capability.
    pub name: String,
    /// Dimension/arity of the capability (e.g., number of inputs it can handle).
    pub arity: usize,
    /// Metadata tags for the capability.
    pub tags: Vec<String>,
}

impl Capability {
    /// Create a new capability.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            arity: 1,
            tags: vec![],
        }
    }

    /// Set the arity of this capability.
    pub fn with_arity(mut self, arity: usize) -> Self {
        self.arity = arity;
        self
    }

    /// Add a tag to this capability.
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Tensor product of two capabilities: parallel composition.
    /// A ⊗ B represents having both capabilities simultaneously.
    pub fn tensor(&self, other: &Self) -> Self {
        Self {
            name: format!("{}⊗{}", self.name, other.name),
            arity: self.arity + other.arity,
            tags: [self.tags.clone(), other.tags.clone()].concat(),
        }
    }

    /// Whether this capability is a unit (identity element for tensor).
    pub fn is_unit(&self) -> bool {
        self.name == "I" && self.arity == 0
    }

    /// The unit capability (identity for tensor product).
    pub fn unit() -> Self {
        Self {
            name: "I".to_string(),
            arity: 0,
            tags: vec![],
        }
    }

    /// The dual of this capability (A* — the "anti-capability" or input type).
    pub fn dual(&self) -> Self {
        Self {
            name: format!("{}*", self.name),
            arity: self.arity,
            tags: self.tags.clone(),
        }
    }

    /// Internal hom: A ⊸ B = A* ⊗ B (the capability of transforming A into B).
    pub fn internal_hom(&self, other: &Self) -> Self {
        self.dual().tensor(other)
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// A capability store: maps agent IDs to their capabilities.
#[derive(Debug, Clone)]
pub struct CapabilityStore {
    /// Map from agent ID to its capability.
    agents: HashMap<String, Capability>,
}

impl CapabilityStore {
    /// Create a new empty store.
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    /// Register an agent with a capability.
    pub fn register(&mut self, agent_id: &str, cap: Capability) {
        self.agents.insert(agent_id.to_string(), cap);
    }

    /// Get the capability of an agent.
    pub fn get(&self, agent_id: &str) -> Option<&Capability> {
        self.agents.get(agent_id)
    }

    /// Find all agents that have a specific capability (by name prefix).
    pub fn find_by_capability(&self, name_prefix: &str) -> Vec<&str> {
        self.agents
            .iter()
            .filter(|(_, cap)| cap.name.starts_with(name_prefix))
            .map(|(id, _)| id.as_str())
            .collect()
    }

    /// Compute the joint capability of two agents (tensor product).
    pub fn joint_capability(&self, a: &str, b: &str) -> Option<Capability> {
        match (self.get(a), self.get(b)) {
            (Some(ca), Some(cb)) => Some(ca.tensor(cb)),
            _ => None,
        }
    }

    /// List all registered agents.
    pub fn agents(&self) -> Vec<&str> {
        self.agents.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_creation() {
        let cap = Capability::new("compute").with_arity(2);
        assert_eq!(cap.name, "compute");
        assert_eq!(cap.arity, 2);
    }

    #[test]
    fn test_tensor_product() {
        let a = Capability::new("sense");
        let b = Capability::new("act");
        let ab = a.tensor(&b);
        assert_eq!(ab.name, "sense⊗act");
        assert_eq!(ab.arity, 2);
    }

    #[test]
    fn test_tensor_identity() {
        let a = Capability::new("compute");
        let unit = Capability::unit();
        let au = a.tensor(&unit);
        assert_eq!(au.arity, 1); // I is identity
        assert!(unit.is_unit());
    }

    #[test]
    fn test_dual() {
        let a = Capability::new("input").with_arity(3);
        let a_dual = a.dual();
        assert_eq!(a_dual.name, "input*");
        assert_eq!(a_dual.arity, 3);
    }

    #[test]
    fn test_internal_hom() {
        let a = Capability::new("sense");
        let b = Capability::new("act");
        let hom = a.internal_hom(&b);
        assert_eq!(hom.name, "sense*⊗act");
    }

    #[test]
    fn test_capability_store() {
        let mut store = CapabilityStore::new();
        store.register("agent-1", Capability::new("compute"));
        store.register("agent-2", Capability::new("sense"));
        assert_eq!(store.agents().len(), 2);
        assert!(store.get("agent-1").is_some());
    }

    #[test]
    fn test_joint_capability() {
        let mut store = CapabilityStore::new();
        store.register("a", Capability::new("compute"));
        store.register("b", Capability::new("sense"));
        let joint = store.joint_capability("a", "b").unwrap();
        assert_eq!(joint.name, "compute⊗sense");
    }

    #[test]
    fn test_find_by_capability() {
        let mut store = CapabilityStore::new();
        store.register("a1", Capability::new("compute"));
        store.register("a2", Capability::new("communicate"));
        store.register("a3", Capability::new("compute-fast"));
        let compute_agents = store.find_by_capability("compute");
        assert_eq!(compute_agents.len(), 2);
    }
}
