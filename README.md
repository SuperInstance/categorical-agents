# categorical-agents

Category theory for multi-agent systems — capabilities as objects, protocols as morphisms, symmetric monoidal categories, and functors.

## Usage

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

// Compose protocols
let composed = cat.compose(&proto1, &proto2);
```

## Features

- **Capabilities as category objects**: Named capabilities with required resources
- **Protocols as morphisms**: Typed communication channels between agent capabilities
- **Symmetric monoidal categories**: Parallel composition via tensor product
- **AgentFunctor**: Structure-preserving maps between agent categories
- **Composition strategies**: Sequential, parallel, and conditional protocol composition

## Tests

31 tests, all passing. `cargo test` to run.

## License

MIT

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance/OpenConstruct) ecosystem.
