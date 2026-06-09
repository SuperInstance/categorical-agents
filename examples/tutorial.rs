//! Tutorial: Categorical Agents — Agent Capabilities as Category Theory
//!
//! Run with: cargo run --example tutorial

use categorical_agents::*;

fn main() {
    println!("=== Lesson 1: Capabilities as Objects ===\n");
    {
        // A Capability is an object in our agent category
        // Think of it as "what an agent can do"
        let compute = Capability::new("compute").with_arity(2);
        let sense = Capability::new("sense").with_tag("environment");
        let unit = Capability::unit();

        println!("Compute capability: {} (arity={})", compute.name, compute.arity);
        println!("Sense capability: {} (tag={:?})", sense.name, sense.tags);
        println!("Unit (identity capability): is_unit={}", unit.is_unit());
        println!();
    }

    println!("=== Lesson 2: Tensor Product (Parallel Composition) ===\n");
    {
        // A ⊗ B = parallel composition of capabilities
        let compute = Capability::new("compute");
        let sense = Capability::new("sense");

        let both = compute.tensor(&sense);
        println!("compute ⊗ sense = {}", both.name);
        println!("This means an agent that can do BOTH in parallel");

        let triple = both.tensor(&Capability::new("communicate"));
        println!("(compute ⊗ sense) ⊗ communicate = {}", triple.name);
        println!();
    }

    println!("=== Lesson 3: Duals and Internal Hom ===\n");
    {
        // Dual: A* = the "inverse" capability
        // Internal hom: [A, B] = "capability to transform A into B"
        let a = Capability::new("audio");
        let b = Capability::new("midi");

        let dual = a.dual();
        println!("Dual of audio: {}", dual.name);

        let hom = a.internal_hom(&b);
        println!("Internal hom [audio, midi] = {}", hom.name);
        println!("This represents a protocol that converts audio → midi");
        println!();
    }

    println!("=== Lesson 4: Protocols as Morphisms ===\n");
    {
        // A Protocol is a morphism from source capability to target capability
        let audio = Capability::new("audio");
        let midi = Capability::new("midi");

        let convert = Protocol::new("audio-to-midi", audio.clone(), midi.clone())
            .with_cost(0.5);

        println!("Protocol: {} (cost={})", convert.name, convert.cost);
        println!("  source: {}", convert.source.name);
        println!("  target: {}", convert.target.name);

        // Identity protocol: does nothing
        let id = Protocol::identity(&audio);
        println!("Identity protocol: {} (is_identity={})", id.name, id.is_identity());
        println!();
    }

    println!("=== Lesson 5: Protocol Composition ===\n");
    {
        // Compose two protocols: if f: A → B and g: B → C, then g∘f: A → C
        let audio = Capability::new("audio");
        let midi = Capability::new("midi");
        let score = Capability::new("score");

        let f = Protocol::new("audio-to-midi", audio.clone(), midi.clone());
        let g = Protocol::new("midi-to-score", midi, score);

        let composed = f.compose(&g).expect("types must match");
        println!("f: audio → midi");
        println!("g: midi → score");
        println!("g∘f: {} → {} (name={})", composed.source.name, composed.target.name, composed.name);

        // Tensor product of protocols
        let p1 = Protocol::new("encode", Capability::new("raw"), Capability::new("encoded"));
        let p2 = Protocol::new("filter", Capability::new("signal"), Capability::new("filtered"));
        let parallel = p1.tensor(&p2);
        println!("\nParallel protocol: {} (runs both simultaneously)", parallel.name);
        println!();
    }

    println!("=== Lesson 6: Building an Agent Category ===\n");
    {
        // An AgentCategory is a collection of capabilities and protocols
        let mut cat = AgentCategory::new();

        let audio = Capability::new("audio");
        let midi = Capability::new("midi");
        let score = Capability::new("score");
        let tensor = Capability::new("tensor");

        cat.add_capability(audio.clone());
        cat.add_capability(midi.clone());
        cat.add_capability(score.clone());
        cat.add_capability(tensor.clone());

        cat.add_protocol(Protocol::new("transcribe", audio.clone(), midi.clone()));
        cat.add_protocol(Protocol::new("render", midi.clone(), score.clone()));
        cat.add_protocol(Protocol::new("analyze", audio.clone(), tensor.clone()));
        cat.add_protocol(Protocol::new("generate", tensor.clone(), midi.clone()));

        println!("Category with {} capabilities, {} protocols",
            cat.capabilities().len(), cat.protocols().len());

        // Find a direct protocol
        if let Some(p) = cat.find_protocol("audio", "midi") {
            println!("Direct protocol audio → midi: {}", p.name);
        }

        // Find a path (composition of protocols)
        if let Some(path) = cat.find_path("audio", "score") {
            println!("Path audio → score: {:?}", path.iter().map(|p| &p.name).collect::<Vec<_>>());
        }

        // Longer path through the graph
        if let Some(path) = cat.find_path("audio", "tensor") {
            println!("Path audio → tensor: {:?}", path.iter().map(|p| &p.name).collect::<Vec<_>>());
        }
        println!();
    }

    println!("=== Lesson 7: Agent Functors ===\n");
    {
        // A Functor maps capabilities and protocols between categories
        let mut functor = AgentFunctor::new("deployment");

        let dev_cap = Capability::new("dev-compute");
        let prod_cap = Capability::new("prod-compute");

        functor.map_object("dev-compute", prod_cap);
        functor.map_object("dev-sense", Capability::new("prod-sense"));

        functor.map_morphism("dev-compute", Protocol::identity(&Capability::new("prod-compute")));

        if let Some(mapped) = functor.apply_capability(&dev_cap) {
            println!("Functor maps dev-compute → {}", mapped.name);
        }

        let valid = functor.verify_composition();
        println!("Functor preserves composition: {}", valid);
        println!();
    }

    println!("=== Lesson 8: Capability Store & Agent Matching ===\n");
    {
        // CapabilityStore is internal — demonstrate composition instead
        let compute = Capability::new("compute").with_arity(4);
        let sense = Capability::new("sense").with_tag("audio");
        let comm = Capability::new("communicate");

        // Tensor: what can multiple agents do together?
        let team_cap = compute.tensor(&sense).tensor(&comm);
        println!("Team capability: {}", team_cap.name);

        // Composition strategies
        let agents = vec![compute.clone(), sense.clone()];
        let mut cat = AgentCategory::new();
        cat.add_capability(compute);
        cat.add_capability(sense.clone());
        let parallel_cap = Composition::parallel(&agents);
        println!("Parallel composition: {}", parallel_cap.name);
        let fan = Composition::fan_out(&sense, 3);
        println!("Fan-out: {}", fan.name);
        println!();
    }

    println!("Tutorial complete! Key takeaway:");
    println!("Category theory gives agents a formal language for composition.");
    println!("Capabilities are objects, protocols are morphisms, and functors");
    println!("map between deployment environments — all type-safe.");
}
