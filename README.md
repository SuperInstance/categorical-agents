# categorical-agents

[![CI](https://github.com/SuperInstance/categorical-agents/actions/workflows/ci.yml/badge.svg)](https://github.com/SuperInstance/categorical-agents/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Composition of agents follows the same laws as composition of functions. You can reason about your fleet the same way you reason about your code.

## The Problem

You have agents. One searches, one summarizes, one acts. How do you compose them? Not with ad-hoc glue code — with *algebra*.

```rust
use categorical_agents::*;
```

---

## 1. Capabilities as Objects

Every agent has a capability — what it *does*. Capabilities are objects in a category.

```rust
use categorical_agents::Capability;

let search    = Capability::new("search").with_arity(1);
let summarize = Capability::new("summarize").with_arity(1);
let act       = Capability::new("act").with_arity(1);

println!("search:    {} (arity {})", search, search.arity);
println!("summarize: {} (arity {})", summarize, summarize.arity);
println!("act:       {} (arity {})", act, act.arity);

// The tensor product: "both at the same time"
let search_and_summarize = search.tensor(&summarize);
println!("{} ⊗ {} = {} (arity {})",
    search, summarize, search_and_summarize, search_and_summarize.arity);
// search ⊗ summarize = search⊗summarize (arity 2)
// This is parallel composition: both capabilities running simultaneously

// The unit: "do nothing"
let unit = Capability::unit();
println!("Unit: {} (arity {})", unit, unit.arity);
// I is the identity for tensor: A ⊗ I = A

// The dual: "the input side of a capability"
let input = Capability::new("input");
let dual = input.dual();
println!("Dual of {}: {}", input, dual);
// input → input* (the anti-capability, the type of data it consumes)

// Internal hom: "how to transform A into B"
let transform = search.internal_hom(&act);
println!("search ⊸ act = {}", transform);
// search*⊗act = "the capability of turning search results into actions"
```

**The tensor product IS parallel execution.** The dual IS the input type. The internal hom IS the transformation.

---

## 2. Protocols as Morphisms

A protocol connects one capability to another. It's a morphism in the category.

```rust
use categorical_agents::{Capability, Protocol};

let raw      = Capability::new("raw");
let encoded  = Capability::new("encoded");
let sent     = Capability::new("sent");
let received = Capability::new("received");

// Protocols: morphisms between capabilities
let encode = Protocol::new("encode", raw, encoded).with_cost(0.5);
let send   = Protocol::new("send", encoded, sent).with_cost(1.0);
let recv   = Protocol::new("receive", sent, received).with_cost(0.3);

println!("encode: {} → {} (cost: {})", encode.source, encode.target, encode.cost);
println!("send:   {} → {} (cost: {})", send.source, send.target, send.cost);

// Compose: g ∘ f means "do f, then do g"
let send_encoded = encode.compose(&send).unwrap();
println!("{} ∘ {} = {} → {} (cost: {})",
    send.name, encode.name, send_encoded.source, send_encoded.target, send_encoded.cost);
// send ∘ encode = raw → sent (cost: 1.5)
// f.target MUST equal g.source — type safety from the math

// Identity: "do nothing" protocol
let id_raw = Protocol::identity(&Capability::new("raw"));
println!("id_raw: {} → {} (cost: {})", id_raw.source, id_raw.target, id_raw.cost);
// id ∘ f = f ∘ id = f
```

**Composition fails when types don't match.** This is the entire point:

```rust
// Try to compose incompatible protocols
let bad = encode.compose(&recv);
match bad {
    Err(msg) => println!("Blocked: {}", msg),
    // "Cannot compose: encode ends at encoded but receive starts at sent"
    Ok(_) => println!("This should not happen"),
}
// The type system prevents nonsensical compositions.
// encode: raw → encoded
// receive: sent → received
// encoded ≠ sent → composition blocked
```

---

## 3. The Category: Objects, Morphisms, and Laws

```rust
use categorical_agents::{AgentCategory, Capability, Protocol};

let mut cat = AgentCategory::new();

// Register capabilities (objects)
cat.add_capability(Capability::new("raw"));
cat.add_capability(Capability::new("parsed"));
cat.add_capability(Capability::new("analyzed"));
cat.add_capability(Capability::new("acted"));

// Register protocols (morphisms)
cat.add_protocol(Protocol::new("parse",   Capability::new("raw"),     Capability::new("parsed")));
cat.add_protocol(Protocol::new("analyze", Capability::new("parsed"),  Capability::new("analyzed")));
cat.add_protocol(Protocol::new("execute", Capability::new("analyzed"),Capability::new("acted")));

// Direct lookup
if let Some(p) = cat.find_protocol("raw", "parsed") {
    println!("Found: {} → {}", p.source, p.target);
}

// Path finding: compose through the chain
if let Some(path) = cat.find_path("raw", "acted") {
    let full = path.iter()
        .map(|p| format!("{}→{}", p.source, p.target))
        .collect::<Vec<_>>()
        .join(" ∘ ");
    println!("Path from raw to acted: {}", full);
    let total_cost: f64 = path.iter().map(|p| p.cost).sum();
    println!("Total cost: {}", total_cost);
}
// raw → parsed → analyzed → acted

// No path exists? Returns None
assert!(cat.find_path("raw", "nonexistent").is_none());
```

### The Structural Isomorphisms

```rust
// Symmetry: swap the order of parallel execution
let sense = Capability::new("sense");
let act = Capability::new("act");
let sigma = cat.symmetry(&sense, &act);
println!("σ: {} → {} (cost: {})", sigma.source, sigma.target, sigma.cost);
// sense⊗act → act⊗sense
// This costs nothing — it's just reordering parallel tasks

// Left unitor: I⊗A = A
let lambda = cat.left_unitor(&sense);
println!("λ: {} → {} (cost: {})", lambda.source, lambda.target, lambda.cost);
// I⊗sense → sense

// Right unitor: A⊗I = A
let rho = cat.right_unitor(&sense);
println!("ρ: {} → {} (cost: {})", rho.source, rho.target, rho.cost);
// sense⊗I → sense
```

---

## 4. Tensor Product: Parallel Composition

```rust
use categorical_agents::{Capability, Protocol, AgentCategory};

// Two agents working in parallel
let fetch  = Protocol::new("fetch",  Capability::new("url"),  Capability::new("html"));
let parse  = Protocol::new("parse",  Capability::new("text"), Capability::new("data"));

// Tensor: both protocols run simultaneously
let parallel = fetch.tensor(&parse);
println!("fetch ⊗ parse: {} → {}", parallel.source, parallel.source);
// url⊗text → html⊗data
println!("Source: {} (arity {})", parallel.source, parallel.source.arity);
println!("Target: {} (arity {})", parallel.target, parallel.target.arity);
println!("Cost: {}", parallel.cost); // sum of both costs
```

**The tensor product of protocols IS parallel execution.** The cost IS the sum. The arity IS the sum.

---

## 5. Capability Store: Your Agent Registry

```rust
use categorical_agents::{Capability, CapabilityStore};

let mut store = CapabilityStore::new();

// Register your fleet
store.register("search-agent",  Capability::new("search").with_tag("retrieval"));
store.register("embed-agent",   Capability::new("embed").with_tag("ml"));
store.register("rank-agent",    Capability::new("rank").with_tag("ml"));
store.register("summary-agent", Capability::new("summarize").with_tag("generation"));
store.register("code-agent",    Capability::new("code").with_tag("generation"));

println!("Registered {} agents", store.agents().len());

// Find agents by capability
let ml_agents = store.find_by_capability("embed");
println!("Embedding agents: {:?}", ml_agents);

let gen_agents = store.find_by_capability("code");
println!("Code agents: {:?}", gen_agents);

// Compute joint capability: what can two agents do together?
if let Some(joint) = store.joint_capability("search-agent", "summary-agent") {
    println!("search + summary = {} (arity {})", joint, joint.arity);
}
// search⊗summarize — they can search AND summarize in parallel
```

---

## 6. Composition Strategies: Sequential, Parallel, Fan-out

```rust
use categorical_agents::{Capability, Protocol, AgentCategory, Composition};

let mut cat = AgentCategory::new();
cat.add_protocol(Protocol::new("fetch",  Capability::new("url"),    Capability::new("html")));
cat.add_protocol(Protocol::new("parse",  Capability::new("html"),   Capability::new("data")));
cat.add_protocol(Protocol::new("embed",  Capability::new("data"),   Capability::new("vectors")));
cat.add_protocol(Protocol::new("search", Capability::new("vectors"),Capability::new("results")));

// Sequential: one after another (pipeline)
let agents = vec![
    Capability::new("url"),
    Capability::new("html"),
    Capability::new("data"),
    Capability::new("vectors"),
    Capability::new("results"),
];
if let Some(pipeline) = Composition::sequential(&agents, &cat) {
    println!("Pipeline: {} → {} (cost: {})",
        pipeline.source, pipeline.target, pipeline.cost);
}
// url → results through the full chain

// Parallel: all at once
let parallel_agents = vec![
    Capability::new("search"),
    Capability::new("summarize"),
    Capability::new("code"),
];
let combined = Composition::parallel(&parallel_agents);
println!("Parallel: {} (arity {})", combined, combined.arity);
// search⊗summarize⊗code

// Fan-out: one capability replicated to N workers
let worker = Capability::new("worker").with_arity(1);
let army = Composition::fan_out(&worker, 5);
println!("Fan-out 5 workers: {} (arity {})", army, army.arity);
// worker⊗worker⊗worker⊗worker⊗worker (arity 5)
```

### Feedback Loops (Trace)

```rust
// A trace: agent A talks to agent B, gets feedback, repeats
let feedback = Protocol::new("iterate",
    Capability::new("state").tensor(&Capability::new("input")),  // A⊗B
    Capability::new("state").tensor(&Capability::new("output")), // A⊗C
);
let cat = AgentCategory::new();
let traced = Composition::trace(&feedback, &cat);
if let Some(t) = traced {
    println!("Traced (feedback loop): {} → {} (cost: {})",
        t.source, t.target, t.cost);
}
// The A part is "fed back" — only B→C remains visible
```

---

## 7. Functors: Map Between Architectures

A functor maps one category to another, preserving structure. Simulation → Production. Testing → Deployment.

```rust
use categorical_agents::{AgentFunctor, Capability, Protocol};

// Functor: translate simulation capabilities to production
let mut sim_to_prod = AgentFunctor::new("deploy");
sim_to_prod.map_object("search-sim",  Capability::new("search-prod"));
sim_to_prod.map_object("embed-sim",   Capability::new("embed-prod"));
sim_to_prod.map_object("rank-sim",    Capability::new("rank-prod"));
sim_to_prod.map_object("code-sim",    Capability::new("code-prod"));

// Map protocols too
sim_to_prod.map_morphism("fetch", Protocol::new(
    "fetch-prod",
    Capability::new("url-prod"),
    Capability::new("html-prod"),
).with_cost(0.1)); // production is faster

// Apply the functor
let sim_cap = Capability::new("search-sim");
if let Some(prod_cap) = sim_to_prod.apply_capability(&sim_cap) {
    println!("{} → {}", sim_cap, prod_cap);
}

// Verify functoriality: F(g∘f) = F(g)∘F(f)
if sim_to_prod.verify_composition() {
    println!("✓ Composition preserved under functor");
}

// Compose functors: sim → staging → production
let mut staging_to_prod = AgentFunctor::new("promote");
staging_to_prod.map_object("embed-prod", Capability::new("embed-v2"));
let full = sim_to_prod.compose(&staging_to_prod);
println!("Full mapping: {} — sim → staging → prod", full.name);
```

---

## The ASCII Diagram

```
Agent Category C:
  Objects: search, summarize, act, ...
  Morphisms: encode, send, parse, ...

  Composition: g ∘ f : A → C  when  f: A → B, g: B → C
  Identity: id_A : A → A
  Tensor: A ⊗ B = parallel(A, B)

Functor F : C → D:
  Maps capabilities to capabilities
  Maps protocols to protocols
  Preserves: identity, composition, tensor

Your fleet:
  search ──parse──▶ data ──embed──▶ vectors ──rank──▶ results
                                                      │
  summary ◀──format── results ◀──────────────────────┘
                              │
  act ◀──decide── analyzed ◀──┘

This IS a category. The protocols ARE morphisms.
Compose them, verify them, optimize them — with algebra.
```

---

## API Reference

| Type | What it does |
|------|-------------|
| `Capability` | An object in the category. Has name, arity, tags. Tensor product = parallel. |
| `Protocol` | A morphism A → B. Composable when types match. Tracks cost. |
| `AgentCategory` | The category itself. Find paths, check isomorphisms, register objects/morphisms. |
| `CapabilityStore` | Agent registry. Find by capability, compute joint capabilities. |
| `Composition` | Strategies: sequential (pipeline), parallel (tensor), fan-out, trace (feedback). |
| `AgentFunctor` | Maps one category to another. Simulation → Production. Preserves structure. |

## Ecosystem

This repo is part of the **SuperInstance** flagship ecosystem — agent-first computation, constraint theory, and self-improving runtimes.

### FLUX Runtime Family

| Repo | Language | Description |
|------|----------|-------------|
| [flux-runtime](https://github.com/SuperInstance/flux-runtime) | Python | Full FLUX runtime: markdown→bytecode, 2037 tests, zero deps |
| [flux-core](https://github.com/SuperInstance/flux-core) | Rust | Register-based bytecode VM, deterministic agent computation |
| [flux-js](https://github.com/SuperInstance/flux-js) | JavaScript | FLUX VM for Node.js and browsers, ~400ns/iter |
| [flux-compiler](https://github.com/SuperInstance/flux-compiler) | Rust/Python | Formal-methods compiler for safety-critical codegen |
| [flux-vm](https://github.com/SuperInstance/flux-vm) | Rust | Stack-based constraint-checking VM, 50 opcodes, Turing-incomplete |

### PLATO Engine Family

| Repo | Language | Description |
|------|----------|-------------|
| [plato-server](https://github.com/SuperInstance/plato-server) | Python | Knowledge tiles, fleet sync via Matrix, HTTP API |
| [plato-engine-block](https://github.com/SuperInstance/plato-engine-block) | Rust | Original room runtime: no_std + alloc, builder pattern |
| [plato-engine-block-c](https://github.com/SuperInstance/plato-engine-block-c) | C99 | Embedded reference: zero heap alloc, bare-metal portable |
| [plato-engine-block-elixir](https://github.com/SuperInstance/plato-engine-block-elixir) | Elixir | BEAM supervision trees, fault tolerance, hot reload |
| [plato-runtime-kernel](https://github.com/SuperInstance/plato-runtime-kernel) | Rust | Spatial model: tensor grid, batons, assertion traps |

### Constraint / Theory Family

| Repo | Language | Description |
|------|----------|-------------|
| [categorical-agents](https://github.com/SuperInstance/categorical-agents) | Rust | Category theory for agent composition (functors, naturality) |
| [cuda-constraint-engine](https://github.com/SuperInstance/cuda-constraint-engine) | CUDA/C | GPU constraint checking at 1B+ constraints/sec |
| [grand-pattern-rs](https://github.com/SuperInstance/grand-pattern-rs) | Rust | Fibonacci dual-direction cellular graph architecture |
| [lau-hodge-theory](https://github.com/SuperInstance/lau-hodge-theory) | Rust | Hodge decomposition, Betti numbers, spectral sequences |
| [ternary-science](https://github.com/SuperInstance/ternary-science) | Rust | Experimental evidence for ternary intelligence, 5 conservation laws |

### Agent / Infrastructure Family

| Repo | Language | Description |
|------|----------|-------------|
| [construct-core](https://github.com/SuperInstance/construct-core) | Rust | Layered trait system: bare-metal → alloc → async agent runtime |
| [crab](https://github.com/SuperInstance/crab) | Bash | Agent shell for repo entry/leave (MUD-room metaphor) |
| [exocortex](https://github.com/SuperInstance/exocortex) | Rust | Persistent cognitive substrate, S3-compatible memory |
| [git-agent](https://github.com/SuperInstance/git-agent) | Python | The repo IS the agent — autonomous lifecycle via Git |
| [capitaine-1](https://github.com/SuperInstance/capitaine-1) | TypeScript | Git-native repo-agent, Cloudflare Workers heartbeat |
| [codespace-edge-rd](https://github.com/SuperInstance/codespace-edge-rd) | Research | Codespace→Edge agent lifecycle and yoke transfer protocols |
| [git-agent-codespace](https://github.com/SuperInstance/git-agent-codespace) | DevContainer | One-click Codespace template for Git-Agent runtimes |

### Registries

| Registry | Package | Install |
|----------|---------|---------|
| **PyPI** | `flux-vm` | `pip install flux-vm` |
| **crates.io** | `fluxvm` | `cargo add fluxvm` |
| **npm** | `flux-js` | `npm install flux-js` *(coming soon)* |

### Philosophy & Architecture

- 📖 [AI-Writings](https://github.com/SuperInstance/AI-Writings) — Philosophy, essays, and design rationale
- 📦 [PACKAGES.md](https://github.com/SuperInstance/SuperInstance/blob/main/PACKAGES.md) — Full package index
