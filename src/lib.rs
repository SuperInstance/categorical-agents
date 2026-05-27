//! # Categorical Agents
//!
//! Formalizes agent capabilities as objects in a symmetric monoidal category.
//! Protocols between agents are morphisms. Capability composition uses tensor products.
//!
//! # Category Theory Concepts
//!
//! - **Objects**: Agent capabilities (e.g., "compute", "sense", "communicate")
//! - **Morphisms**: Protocols/transformations between capabilities
//! - **Tensor product**: Parallel composition of capabilities (A ⊗ B)
//! - **Hom-set**: All possible protocols from A to B: Hom(A, B)
//! - **Natural transformations**: Mappings between agent architectures

mod capability;
mod category;
mod composition;
mod functor;
mod protocol;

pub use capability::Capability;
pub use category::AgentCategory;
pub use composition::Composition;
pub use functor::AgentFunctor;
pub use protocol::Protocol;
