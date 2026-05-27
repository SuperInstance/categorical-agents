//! Protocols as morphisms in the agent category.

use crate::capability::Capability;

/// A protocol: a morphism from source capability to target capability.
///
/// In category theory terms: f : A → B where A and B are capabilities.
#[derive(Debug, Clone)]
pub struct Protocol {
    /// Source capability.
    pub source: Capability,
    /// Target capability.
    pub target: Capability,
    /// Protocol name/identifier.
    pub name: String,
    /// Cost of executing this protocol (for optimization).
    pub cost: f64,
}

impl Protocol {
    /// Create a new protocol from source to target.
    pub fn new(name: &str, source: Capability, target: Capability) -> Self {
        Self {
            source,
            target,
            name: name.to_string(),
            cost: 1.0,
        }
    }

    /// Set the cost of this protocol.
    pub fn with_cost(mut self, cost: f64) -> Self {
        self.cost = cost;
        self
    }

    /// Compose two protocols: (g ∘ f)(x) = g(f(x)).
    /// Requires f.target == g.source.
    pub fn compose(&self, other: &Self) -> Result<Protocol, String> {
        if self.target != other.source {
            return Err(format!(
                "Cannot compose: {} ends at {} but {} starts at {}",
                self.name, self.target, other.name, other.source
            ));
        }
        Ok(Protocol::new(
            &format!("{}∘{}", other.name, self.name),
            self.source.clone(),
            other.target.clone(),
        )
        .with_cost(self.cost + other.cost))
    }

    /// Identity protocol: id_A : A → A.
    pub fn identity(cap: &Capability) -> Self {
        Self {
            source: cap.clone(),
            target: cap.clone(),
            name: format!("id_{}", cap.name),
            cost: 0.0,
        }
    }

    /// Tensor product of two protocols.
    /// (f ⊗ g) : A⊗C → B⊗D where f:A→B and g:C→D.
    pub fn tensor(&self, other: &Self) -> Protocol {
        Protocol::new(
            &format!("{}⊗{}", self.name, other.name),
            self.source.tensor(&other.source),
            self.target.tensor(&other.target),
        )
        .with_cost(self.cost + other.cost)
    }

    /// Whether this is an identity morphism.
    pub fn is_identity(&self) -> bool {
        self.source == self.target && self.cost == 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_creation() {
        let p = Protocol::new(
            "encode",
            Capability::new("data"),
            Capability::new("encoded"),
        );
        assert_eq!(p.name, "encode");
    }

    #[test]
    fn test_composition() {
        let f = Protocol::new("encode", Capability::new("raw"), Capability::new("encoded"));
        let g = Protocol::new("send", Capability::new("encoded"), Capability::new("sent"));
        let gf = f.compose(&g).unwrap();
        assert_eq!(gf.source, Capability::new("raw"));
        assert_eq!(gf.target, Capability::new("sent"));
        assert_eq!(gf.name, "send∘encode");
    }

    #[test]
    fn test_composition_fails_wrong_types() {
        let f = Protocol::new("a", Capability::new("x"), Capability::new("y"));
        let g = Protocol::new("b", Capability::new("z"), Capability::new("w"));
        assert!(f.compose(&g).is_err());
    }

    #[test]
    fn test_identity() {
        let cap = Capability::new("data");
        let id = Protocol::identity(&cap);
        assert!(id.is_identity());
        assert_eq!(id.source, id.target);
    }

    #[test]
    fn test_tensor_product() {
        let f = Protocol::new("a", Capability::new("x"), Capability::new("y"));
        let g = Protocol::new("b", Capability::new("u"), Capability::new("v"));
        let fg = f.tensor(&g);
        assert_eq!(fg.source.name, "x⊗u");
        assert_eq!(fg.target.name, "y⊗v");
    }

    #[test]
    fn test_cost_tracking() {
        let f = Protocol::new("a", Capability::new("x"), Capability::new("y")).with_cost(2.5);
        let g = Protocol::new("b", Capability::new("y"), Capability::new("z")).with_cost(3.0);
        let gf = f.compose(&g).unwrap();
        assert_eq!(gf.cost, 5.5);
    }
}
