//! Agent category: the symmetric monoidal category of agent capabilities.

use crate::capability::Capability;
use crate::protocol::Protocol;
use std::collections::HashMap;

/// The category of agent capabilities.
///
/// - **Objects**: Capabilities
/// - **Morphisms**: Protocols between capabilities
/// - **Monoidal structure**: Tensor product (parallel composition)
/// - **Symmetry**: Swap morphism σ : A⊗B → B⊗A
pub struct AgentCategory {
    /// All known capabilities (objects).
    capabilities: Vec<Capability>,
    /// All known protocols (morphisms), indexed by (source_name, target_name).
    protocols: HashMap<(String, String), Protocol>,
}

impl AgentCategory {
    /// Create an empty category.
    pub fn new() -> Self {
        Self {
            capabilities: vec![Capability::unit()],
            protocols: HashMap::new(),
        }
    }

    /// Add a capability (object) to the category.
    pub fn add_capability(&mut self, cap: Capability) {
        if !self.capabilities.iter().any(|c| c.name == cap.name) {
            self.capabilities.push(cap);
        }
    }

    /// Add a protocol (morphism) to the category.
    pub fn add_protocol(&mut self, proto: Protocol) {
        self.add_capability(proto.source.clone());
        self.add_capability(proto.target.clone());
        self.protocols.insert(
            (proto.source.name.clone(), proto.target.name.clone()),
            proto,
        );
    }

    /// Find a protocol from source to target (direct morphism).
    pub fn find_protocol(&self, source: &str, target: &str) -> Option<&Protocol> {
        self.protocols
            .get(&(source.to_string(), target.to_string()))
    }

    /// Find a path of protocols from source to target via composition.
    /// Uses BFS on the protocol graph.
    pub fn find_path(&self, source: &str, target: &str) -> Option<Vec<Protocol>> {
        if source == target {
            let cap = self.capabilities.iter().find(|c| c.name == source)?;
            return Some(vec![Protocol::identity(cap)]);
        }

        // BFS
        let mut queue = vec![(
            source.to_string(),
            vec![Protocol::identity(&Capability::new(source))],
        )];
        let mut visited = vec![source.to_string()];

        while let Some((current, path)) = queue.pop() {
            // Find all protocols starting from current
            for ((s, t), proto) in &self.protocols {
                if s == &current && !visited.contains(t) {
                    let mut new_path = path.clone();
                    if !path.last().unwrap().is_identity() {
                        let composed = path.last().unwrap().compose(proto).ok()?;
                        new_path.pop();
                        new_path.push(composed);
                    } else {
                        new_path = vec![proto.clone()];
                    }

                    if t == target {
                        return Some(new_path);
                    }

                    visited.push(t.clone());
                    queue.push((t.clone(), new_path));
                }
            }
        }
        None
    }

    /// Symmetry isomorphism: σ : A⊗B → B⊗A.
    pub fn symmetry(&self, a: &Capability, b: &Capability) -> Protocol {
        Protocol::new(
            &format!("σ_{}_{}", a.name, b.name),
            a.tensor(b),
            b.tensor(a),
        )
        .with_cost(0.0) // symmetry is natural, zero cost
    }

    /// Left unitor: λ_A : I⊗A → A.
    pub fn left_unitor(&self, a: &Capability) -> Protocol {
        Protocol::new(
            &format!("λ_{}", a.name),
            Capability::unit().tensor(a),
            a.clone(),
        )
        .with_cost(0.0)
    }

    /// Right unitor: ρ_A : A⊗I → A.
    pub fn right_unitor(&self, a: &Capability) -> Protocol {
        Protocol::new(
            &format!("ρ_{}", a.name),
            a.tensor(&Capability::unit()),
            a.clone(),
        )
        .with_cost(0.0)
    }

    /// List all capabilities.
    pub fn capabilities(&self) -> &[Capability] {
        &self.capabilities
    }

    /// List all protocols.
    pub fn protocols(&self) -> &HashMap<(String, String), Protocol> {
        &self.protocols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_category() {
        let cat = AgentCategory::new();
        assert!(cat.capabilities().len() >= 1); // unit
    }

    #[test]
    fn test_add_capability() {
        let mut cat = AgentCategory::new();
        cat.add_capability(Capability::new("compute"));
        assert!(cat.capabilities().iter().any(|c| c.name == "compute"));
    }

    #[test]
    fn test_add_protocol() {
        let mut cat = AgentCategory::new();
        cat.add_protocol(Protocol::new(
            "encode",
            Capability::new("raw"),
            Capability::new("encoded"),
        ));
        assert!(cat.find_protocol("raw", "encoded").is_some());
    }

    #[test]
    fn test_symmetry() {
        let cat = AgentCategory::new();
        let a = Capability::new("sense");
        let b = Capability::new("act");
        let sigma = cat.symmetry(&a, &b);
        assert_eq!(sigma.source.name, "sense⊗act");
        assert_eq!(sigma.target.name, "act⊗sense");
        assert_eq!(sigma.cost, 0.0);
    }

    #[test]
    fn test_left_unitor() {
        let cat = AgentCategory::new();
        let a = Capability::new("compute");
        let lambda = cat.left_unitor(&a);
        assert_eq!(lambda.target, a);
    }

    #[test]
    fn test_find_path_direct() {
        let mut cat = AgentCategory::new();
        cat.add_protocol(Protocol::new(
            "a",
            Capability::new("x"),
            Capability::new("y"),
        ));
        cat.add_protocol(Protocol::new(
            "b",
            Capability::new("y"),
            Capability::new("z"),
        ));
        let path = cat.find_path("x", "z");
        assert!(path.is_some());
    }

    #[test]
    fn test_find_path_identity() {
        let mut cat = AgentCategory::new();
        cat.add_capability(Capability::new("x"));
        let path = cat.find_path("x", "x");
        assert!(path.is_some());
        assert!(path.unwrap()[0].is_identity());
    }
}
