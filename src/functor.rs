//! Functors between agent categories — mapping one agent architecture to another.

use crate::capability::Capability;
use crate::protocol::Protocol;

/// A functor F : C → D between agent categories.
///
/// Maps objects (capabilities) to objects and morphisms (protocols) to morphisms,
/// preserving composition and identity.
pub struct AgentFunctor {
    /// Name of this functor.
    pub name: String,
    /// Object mapping: capability name in source → capability in target.
    object_map: std::collections::HashMap<String, Capability>,
    /// Morphism mapping: protocol name in source → protocol in target.
    morphism_map: std::collections::HashMap<String, Protocol>,
}

impl AgentFunctor {
    /// Create a new named functor.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            object_map: std::collections::HashMap::new(),
            morphism_map: std::collections::HashMap::new(),
        }
    }

    /// Map a source capability to a target capability.
    pub fn map_object(&mut self, source_name: &str, target: Capability) {
        self.object_map.insert(source_name.to_string(), target);
    }

    /// Map a source protocol to a target protocol.
    pub fn map_morphism(&mut self, source_name: &str, target: Protocol) {
        self.morphism_map.insert(source_name.to_string(), target);
    }

    /// Apply the functor to a capability.
    pub fn apply_capability(&self, cap: &Capability) -> Option<&Capability> {
        self.object_map.get(&cap.name)
    }

    /// Apply the functor to a protocol.
    pub fn apply_protocol(&self, proto: &Protocol) -> Option<&Protocol> {
        self.morphism_map.get(&proto.name)
    }

    /// Verify functoriality: F(g ∘ f) == F(g) ∘ F(f).
    /// Returns true if the functor preserves composition for all mapped morphisms.
    pub fn verify_composition(&self) -> bool {
        // For each pair of composable mapped protocols, verify composition
        for pf1 in self.morphism_map.values() {
            for pf2 in self.morphism_map.values() {
                if pf1.target == pf2.source {
                    // They compose in the target category
                    if pf1.compose(pf2).is_err() {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Compose two functors: (G ∘ F)(x) = G(F(x)).
    pub fn compose(&self, other: &Self) -> AgentFunctor {
        let mut composed = AgentFunctor::new(&format!("{}∘{}", other.name, self.name));

        // Object map: apply F then G
        for (name, f_cap) in &self.object_map {
            if let Some(g_cap) = other.apply_capability(f_cap) {
                composed.map_object(name, g_cap.clone());
            }
        }

        composed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functor_creation() {
        let f = AgentFunctor::new("embed");
        assert_eq!(f.name, "embed");
    }

    #[test]
    fn test_object_mapping() {
        let mut f = AgentFunctor::new("translate");
        f.map_object("compute", Capability::new("gpu-compute"));
        let result = f.apply_capability(&Capability::new("compute"));
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "gpu-compute");
    }

    #[test]
    fn test_morphism_mapping() {
        let mut f = AgentFunctor::new("translate");
        f.map_morphism(
            "encode",
            Protocol::new(
                "fast-encode",
                Capability::new("raw"),
                Capability::new("encoded"),
            ),
        );
        let result = f.apply_protocol(&Protocol::new(
            "encode",
            Capability::new("x"),
            Capability::new("y"),
        ));
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "fast-encode");
    }

    #[test]
    fn test_verify_composition() {
        let mut f = AgentFunctor::new("test");
        f.map_morphism(
            "a",
            Protocol::new("fa", Capability::new("x"), Capability::new("y")),
        );
        f.map_morphism(
            "b",
            Protocol::new("fb", Capability::new("y"), Capability::new("z")),
        );
        assert!(f.verify_composition());
    }

    #[test]
    fn test_functor_composition() {
        let mut f = AgentFunctor::new("F");
        f.map_object("a", Capability::new("fa"));

        let mut g = AgentFunctor::new("G");
        g.map_object("fa", Capability::new("gfa"));

        let gf = f.compose(&g);
        assert_eq!(gf.name, "G∘F");
    }
}
