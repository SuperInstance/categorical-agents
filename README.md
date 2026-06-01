# categorical-agents

**Category theory for agent capabilities: objects as capabilities, morphisms as protocols, composition as tensor products.**

A Rust library that formalizes multi-agent systems as a symmetric monoidal category. Capabilities are objects, protocols are morphisms, and agent coordination reduces to categorical composition.

## What This Does

This library models agent systems using category theory:

1. **Capabilities as objects** — Each agent has a capability (compute, sense, act). Capabilities compose via tensor product.
2. **Protocols as morphisms** — A protocol transforms one capability into another. Protocols compose (sequentially) and tensor (in parallel).
3. **Categories as architecture** — The full collection of capabilities and protocols forms a symmetric monoidal category with associators, unitors, and symmetry isomorphisms.
4. **Functors as architecture translations** — Map one agent architecture to another while preserving structure.
5. **Composition strategies** — Sequential, parallel, fan-out, and trace (feedback loop) composition.

## Key Idea

In a symmetric monoidal category (SMC), you get:
- **Objects** (capabilities) with a tensor product ⊗ for parallel composition
- **Morphisms** (protocols) f: A → B that compose sequentially
- **Structural isomorphisms**: associativity α, symmetry σ, and unitors λ, ρ

This library treats agent fleets as such a category. The payoff: type-safe capability composition, protocol verification via functoriality checks, and a mathematical framework for reasoning about multi-agent coordination.

## Install

```toml
[dependencies]
categorical-agents = "0.1.0"
```

Or:

```sh
cargo add categorical-agents
```

Requires Rust 2021 edition. No external dependencies.

## Quick Start

### Capabilities and Tensor Products

```rust
use categorical_agents::Capability;

let sense = Capability::new("sense").with_arity(1);
let compute = Capability::new("compute").with_arity(2);
let act = Capability::new("act");

// Parallel composition: having both capabilities
let combined = sense.tensor(&compute);
assert_eq!(combined.name, "sense⊗compute");
assert_eq!(combined.arity, 3);

// The unit capability I (identity for tensor)
let unit = Capability::unit();
let same = sense.tensor(&unit); // ≅ sense

// Internal hom: "capability of transforming sense into act"
let hom = sense.internal_hom(&act); // sense*⊗act
```

### Protocols as Morphisms

```rust
use categorical_agents::{Capability, Protocol};

let f = Protocol::new("encode", Capability::new("raw"), Capability::new("encoded"))
    .with_cost(2.0);
let g = Protocol::new("send", Capability::new("encoded"), Capability::new("sent"))
    .with_cost(3.0);

// Sequential composition: g ∘ f
let gf = f.compose(&g).unwrap();
assert_eq!(gf.source, Capability::new("raw"));
assert_eq!(gf.target, Capability::new("sent"));
assert_eq!(gf.cost, 5.0); // costs add

// Parallel composition: f ⊗ g
let fg = f.tensor(&g);
// source: raw⊗encoded, target: encoded⊗sent

// Identity morphism
let id = Protocol::identity(&Capability::new("data"));
assert!(id.is_identity());
```

### The Agent Category

```rust
use categorical_agents::{AgentCategory, Capability, Protocol};

let mut cat = AgentCategory::new();

// Add protocols (capabilities auto-registered)
cat.add_protocol(Protocol::new("sense-raw", Capability::new("sensor"), Capability::new("raw")));
cat.add_protocol(Protocol::new("raw-processed", Capability::new("raw"), Capability::new("processed")));
cat.add_protocol(Protocol::new("processed-action", Capability::new("processed"), Capability::new("action")));

// Find a path through the category (BFS composition)
let path = cat.find_path("sensor", "action");
assert!(path.is_some());

// Structural isomorphisms
let a = Capability::new("sense");
let b = Capability::new("act");
let sigma = cat.symmetry(&a, &b); // σ: sense⊗act → act⊗sense
let lambda = cat.left_unitor(&a);  // λ: I⊗sense → sense
let rho = cat.right_unitor(&a);    // ρ: sense⊗I → sense
```

### Functors Between Architectures

```rust
use categorical_agents::{AgentFunctor, Capability, Protocol};

let mut embed = AgentFunctor::new("embed");
embed.map_object("compute", Capability::new("gpu-compute"));
embed.map_object("sense", Capability::new("lidar-sense"));

// Apply functor to a capability
let mapped = embed.apply_capability(&Capability::new("compute"));
assert_eq!(mapped.unwrap().name, "gpu-compute");

// Compose functors: G ∘ F
let mut translate = AgentFunctor::new("translate");
translate.map_object("gpu-compute", Capability::new("tpu-compute"));
let composed = embed.compose(&translate);
```

### Composition Strategies

```rust
use categorical_agents::{Capability, Composition, AgentCategory, Protocol};

let mut cat = AgentCategory::new();
cat.add_protocol(Protocol::new("a", Capability::new("x"), Capability::new("y")));
cat.add_protocol(Protocol::new("b", Capability::new("y"), Capability::new("z")));

// Sequential: chain agents via protocols
let agents = vec![Capability::new("x"), Capability::new("y"), Capability::new("z")];
let chain = Composition::sequential(&agents, &cat);

// Parallel: tensor all capabilities
let parallel = Composition::parallel(&agents);
assert_eq!(parallel.name, "x⊗y⊗z");

// Fan-out: replicate a capability n times
let workers = Composition::fan_out(&Capability::new("worker"), 4);
assert_eq!(workers.arity, 4);
```

### Capability Store

```rust
use categorical_agents::{Capability, CapabilityStore};

let mut store = CapabilityStore::new();
store.register("agent-1", Capability::new("compute"));
store.register("agent-2", Capability::new("sense"));
store.register("agent-3", Capability::new("communicate"));

// Find agents by capability prefix
let compute_agents = store.find_by_capability("compute"); // agent-1

// Joint capability via tensor product
let joint = store.joint_capability("agent-1", "agent-2").unwrap();
assert_eq!(joint.name, "compute⊗sense");
```

## API Reference

### `Capability`

An object in the capability category.

| Method | Description |
|--------|-------------|
| `new(name)` | Create a capability |
| `with_arity(n)` | Set input arity |
| `with_tag(tag)` | Add metadata tag |
| `tensor(other)` | Parallel composition A ⊗ B |
| `dual()` | Anti-capability A* |
| `internal_hom(other)` | Linear hom A ⊸ B = A* ⊗ B |
| `is_unit()` | Check if identity element I |
| `unit()` | The unit capability I |

### `Protocol`

A morphism f: A → B between capabilities.

| Method | Description |
|--------|-------------|
| `new(name, source, target)` | Create a protocol |
| `with_cost(c)` | Set execution cost |
| `compose(other)` | Sequential composition g ∘ f |
| `tensor(other)` | Parallel composition f ⊗ g |
| `identity(cap)` | Identity morphism id_A |
| `is_identity()` | Check if identity |

### `AgentCategory`

The symmetric monoidal category of capabilities and protocols.

| Method | Description |
|--------|-------------|
| `new()` | Empty category (with unit object) |
| `add_capability(cap)` | Add an object |
| `add_protocol(proto)` | Add a morphism |
| `find_protocol(src, tgt)` | Direct morphism lookup |
| `find_path(src, tgt)` | BFS path finding via composition |
| `symmetry(a, b)` | σ: A⊗B → B⊗A |
| `left_unitor(a)` | λ: I⊗A → A |
| `right_unitor(a)` | ρ: A⊗I → A |

### `AgentFunctor`

A structure-preserving map between agent categories.

| Method | Description |
|--------|-------------|
| `new(name)` | Create a named functor |
| `map_object(src, target)` | Map capability → capability |
| `map_morphism(src, target)` | Map protocol → protocol |
| `apply_capability(cap)` | Apply F to an object |
| `apply_protocol(proto)` | Apply F to a morphism |
| `verify_composition()` | Check F(g∘f) == F(g)∘F(f) |
| `compose(other)` | Functor composition G ∘ F |

### `Composition`

Static methods for composition strategies.

| Method | Description |
|--------|-------------|
| `sequential(agents, cat)` | Chain agents via protocols |
| `parallel(agents)` | Tensor all capabilities |
| `fan_out(cap, n)` | Replicate capability n times |
| `trace(protocol, cat)` | Feedback loop (categorical trace) |

### `CapabilityStore`

A registry mapping agent IDs to capabilities.

| Method | Description |
|--------|-------------|
| `new()` | Empty store |
| `register(id, cap)` | Register an agent |
| `get(id)` | Lookup by agent ID |
| `find_by_capability(prefix)` | Find agents by capability name |
| `joint_capability(a, b)` | Tensor product of two agents' capabilities |

## How It Works

### Symmetric Monoidal Category Structure

The library encodes a strict symmetric monoidal category (SMC):

- **Objects**: `Capability` values identified by name, with arity tracking the number of parallel inputs.
- **Morphisms**: `Protocol` values with source/target capabilities, a name, and a cost.
- **Tensor product**: `Capability::tensor` and `Protocol::tensor` for parallel composition. The unit is `Capability::unit()` (arity 0, name "I").
- **Composition**: `Protocol::compose` for sequential composition, requiring target-match (f.target == g.source).
- **Structural isomorphisms**: symmetry σ, left unitor λ, right unitor ρ — all zero-cost protocols.

### Path Finding

`AgentCategory::find_path` uses BFS over the protocol graph to find a compositional chain of morphisms from source to target capability. Identity morphisms handle the trivial case (source == target).

### Functorial Verification

`AgentFunctor::verify_composition` checks that for all pairs of composable mapped protocols, composition is preserved: F(g ∘ f) = F(g) ∘ F(f).

### Trace (Feedback)

`Composition::trace` implements a simplified categorical trace: given a protocol f: A⊗B → A⊗C, it produces Tr(f): B → C by "feeding back" the A component. This models feedback loops in agent systems.

## The Math

### Symmetric Monoidal Category

A symmetric monoidal category (C, ⊗, I, α, λ, ρ, σ) consists of:
- A category C with objects and morphisms
- A tensor product ⊗: C × C → C that is associative (up to α) and has unit I
- Symmetry σ_{A,B}: A ⊗ B → B ⊗ A (swap)
- Coherence conditions: the triangle and pentagon equations

### Capabilities as Objects

Each capability A has an arity (dimension) and the tensor adds arities: arity(A ⊗ B) = arity(A) + arity(B). The unit I has arity 0.

### Protocols as Morphisms

A protocol f: A → B is a morphism. Sequential composition g ∘ f: A → C requires f: A → B and g: B → C. Costs are additive: cost(g ∘ f) = cost(g) + cost(f).

### Tensor Product of Morphisms

Given f: A → B and g: C → D, the tensor f ⊗ g: A⊗C → B⊗D applies both morphisms in parallel.

### Internal Hom (Linear Implication)

The internal hom A ⊸ B = A* ⊗ B represents the capability of "transforming A into B". This is the currying/linear logic interpretation of the monoidal structure.

### Functors

A functor F: C → D maps objects to objects and morphisms to morphisms, preserving:
- Identity: F(id_A) = id_{F(A)}
- Composition: F(g ∘ f) = F(g) ∘ F(f)
- Monoidal structure: F(A ⊗ B) ≅ F(A) ⊗ F(B), F(I) ≅ I

### Trace

The categorical trace Tr^A(f: A⊗B → A⊗C): B → C "feeds back" the A output to the A input. In traced monoidal categories, this gives a canonical notion of feedback/iteration.

## Test Coverage

31 tests across 5 modules:
- **Capability** (8): creation, tensor product, unit identity, dual, internal hom, store operations, joint capabilities, prefix search
- **Category** (7): empty category, add capability/protocol, symmetry, left/right unitor, path finding (direct + identity)
- **Composition** (5): parallel, fan-out, sequential with protocols, minimum-length check, trace
- **Functor** (5): creation, object mapping, morphism mapping, composition verification, functor composition
- **Protocol** (6): creation, composition, type mismatch rejection, identity, tensor product, cost tracking

## License

MIT
