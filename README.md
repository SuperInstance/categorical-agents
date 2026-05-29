# categorical-agents

Category theory for multi-agent systems — capabilities as objects, protocols as morphisms, symmetric monoidal categories, and functors for structure-preserving maps between agent groups.

## What This Gives You

- **Capabilities as objects** — Named capabilities with required resources, the "things" agents can do
- **Protocols as morphisms** — Typed communication channels between agent capabilities
- **Symmetric monoidal categories** — Parallel composition via tensor product (⊗)
- **`AgentFunctor`** — Structure-preserving maps between agent categories
- **Composition strategies** — Sequential, parallel, and conditional protocol composition

## Quick Start

```rust
use categorical_agents::{AgentCategory, Capability, Protocol, AgentFunctor};

// Define capabilities
let sensing = Capability::new("sensing", vec!["lidar", "camera"]);
let planning = Capability::new("planning", vec!["path", "trajectory"]);

// Create a category
let mut cat = AgentCategory::new();
cat.add_object(sensing.clone());
cat.add_object(planning.clone());

// Add a protocol (morphism) between capabilities
let perceive = Protocol::new("perceive", sensing.clone(), planning.clone());
cat.add_morphism(perceive);

// Tensor product: compose capabilities in parallel
let tensor = cat.tensor(&sensing, &planning);

// Functor: map one agent category into another
let mut target_cat = AgentCategory::new();
let functor = AgentFunctor::new(&cat, &target_cat);
```

## API Reference

### `Capability`

```rust
Capability::new(name, resources)  // Create a named capability with required resources
```

### `Protocol`

```rust
Protocol::new(name, source, target)  // Morphism between two capabilities
```

### `AgentCategory`

| Method | Description |
|--------|-------------|
| `new()` | Empty category |
| `add_object(capability)` | Add a capability object |
| `add_morphism(protocol)` | Add a protocol morphism |
| `compose(&proto1, &proto2)` | Sequential composition |
| `tensor(&cap1, &cap2)` | Parallel composition (⊗) |

### `AgentFunctor`

```rust
AgentFunctor::new(source_category, target_category)
// Maps objects → objects and morphisms → morphisms, preserving structure
```

## How It Fits

- **[capability-spec-rs](https://github.com/SuperInstance/capability-spec-rs)** — Capability specs define the objects; categorical-agents provides the algebra
- **[categorical-agents-c](https://github.com/SuperInstance/categorical-agents-c)** — C99 port for embedded/bare-metal agent systems
- **[conservation-protocol](https://github.com/SuperInstance/conservation-protocol)** — Spectral fingerprints for verifying functor structure preservation
- **[co-captain-git-agent](https://github.com/SuperInstance/co-captain-git-agent)** — Fleet composition uses category-theoretic composition

## Testing

31 tests covering object/morphism creation, identity laws, associativity, tensor product, functor mapping, and composition strategies.

```bash
cargo test
```

## Installation

```toml
[dependencies]
categorical-agents = { git = "https://github.com/SuperInstance/categorical-agents" }
```

```bash
git clone https://github.com/SuperInstance/categorical-agents.git
cd categorical-agents
cargo build
```

## License

MIT

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance) ecosystem.
