//! Composition strategies for multi-agent systems.

use crate::capability::Capability;
use crate::category::AgentCategory;
use crate::protocol::Protocol;

/// Strategies for composing agent capabilities.
pub struct Composition;

impl Composition {
    /// Sequential composition: agents execute in order.
    /// A₁ → A₂ → ... → Aₙ as a chain of protocols.
    pub fn sequential(agents: &[Capability], cat: &AgentCategory) -> Option<Protocol> {
        if agents.len() < 2 {
            return None;
        }
        let mut result = cat.find_protocol(&agents[0].name, &agents[1].name)?.clone();
        for i in 2..agents.len() {
            result = result
                .compose(cat.find_protocol(&agents[i - 1].name, &agents[i].name)?)
                .ok()?;
        }
        Some(result)
    }

    /// Parallel composition: all agents act simultaneously.
    /// A₁ ⊗ A₂ ⊗ ... ⊗ Aₙ
    pub fn parallel(agents: &[Capability]) -> Capability {
        agents
            .iter()
            .skip(1)
            .fold(agents[0].clone(), |acc, cap| acc.tensor(cap))
    }

    /// Fan-out: one capability replicated to n targets.
    pub fn fan_out(cap: &Capability, n: usize) -> Capability {
        (0..n - 1).fold(cap.clone(), |acc, _| acc.tensor(cap))
    }

    /// Feedback loop: A → B → A (a trace in category theory).
    /// Tr^A_B(f : A⊗B → A⊗C) : B → C
    pub fn trace(f: &Protocol, cat: &AgentCategory) -> Option<Protocol> {
        // Simplified: if f : A⊗B → A⊗C, then Tr(f) : B → C
        // by "feeding back" the A output to the A input
        if f.source.name.contains('⊗') && f.target.name.contains('⊗') {
            Some(
                Protocol::new(
                    &format!("Tr({})", f.name),
                    Capability::new("feedback-source"),
                    Capability::new("feedback-target"),
                )
                .with_cost(f.cost),
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_composition() {
        let agents = vec![
            Capability::new("sense"),
            Capability::new("compute"),
            Capability::new("act"),
        ];
        let combined = Composition::parallel(&agents);
        assert_eq!(combined.name, "sense⊗compute⊗act");
        assert_eq!(combined.arity, 3);
    }

    #[test]
    fn test_fan_out() {
        let cap = Capability::new("worker").with_arity(1);
        let fanned = Composition::fan_out(&cap, 3);
        assert_eq!(fanned.arity, 3);
    }

    #[test]
    fn test_sequential_needs_protocols() {
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
        let agents = vec![
            Capability::new("x"),
            Capability::new("y"),
            Capability::new("z"),
        ];
        let result = Composition::sequential(&agents, &cat);
        assert!(result.is_some());
        let proto = result.unwrap();
        assert_eq!(proto.target, Capability::new("z"));
    }

    #[test]
    fn test_sequential_needs_two() {
        let cat = AgentCategory::new();
        let agents = vec![Capability::new("x")];
        assert!(Composition::sequential(&agents, &cat).is_none());
    }

    #[test]
    fn test_trace() {
        let f = Protocol::new("loop", Capability::new("A⊗B"), Capability::new("A⊗C"));
        let cat = AgentCategory::new();
        let traced = Composition::trace(&f, &cat);
        assert!(traced.is_some());
    }
}
