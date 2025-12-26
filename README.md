# LangSmith Rust

A production-ready Rust crate for manual tracing to LangSmith, providing similar ergonomics to the Python and TypeScript SDKs. Designed for building observable AI agent systems with LangGraph-like architectures.

## Features

- **Automatic Configuration** - Environment-based setup with `.env` support
- **Hierarchical Tracing** - Parent-child run relationships with automatic context propagation
- **Agent Message Types** - Full support for tool calls, tool messages, AI messages, and human messages
- **Thread Management** - Conversation and session tracking
- **Metrics Support** - Token counting and cost tracking
- **Non-blocking Async** - All tracing operations are async and non-blocking
- **Decorator Pattern** - Automatic node tracing with `trace_node` helper
- **Design Patterns** - Strategy, Factory, and Observer patterns for extensibility
- **Type Safety** - Full Rust type safety with compile-time guarantees

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
langsmith-rust = { path = "../langsmith-rust" }
```

Or from a git repository:

```toml
[dependencies]
langsmith-rust = { git = "https://github.com/your-org/langsmith-rust" }
```

## Quick Start

### 1. Configuration

Create a `.env` file in your project root:

```bash
LANGSMITH_TRACING=true
LANGSMITH_ENDPOINT=https://api.smith.langchain.com
LANGSMITH_API_KEY=<your-api-key>
LANGSMITH_PROJECT=<your-project-name>
LANGSMITH_TENANT_ID=<workspace-id>  # Optional
```

### 2. Initialize

```rust
use langsmith_rust;

// Initialize (loads .env automatically)
langsmith_rust::init();
```

### 3. Basic Usage

#### Manual Tracing

```rust
use langsmith_rust::{Tracer, RunType};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    langsmith_rust::init();
    
    // Create root tracer
    let mut tracer = Tracer::new(
        "Agent Pipeline",
        RunType::Chain,
        json!({"question": "What is Rust?"})
    ).with_thread_id("thread-123".to_string());
    
    // Post initial run
    tracer.post().await?;
    
    // Create child run for LLM call
    let mut llm_run = tracer.create_child(
        "OpenAI Call",
        RunType::Llm,
        json!({"messages": [{"role": "user", "content": "What is Rust?"}]})
    );
    llm_run.post().await?;
    
    // ... execute LLM call ...
    let completion = "Rust is a systems programming language...";
    
    // Update run with outputs
    llm_run.end(json!({"completion": completion}));
    llm_run.patch().await?;
    
    // End parent run
    tracer.end(json!({"answer": completion}));
    tracer.patch().await?;
    
    Ok(())
}
```

#### Automatic Node Tracing (Recommended)

```rust
use langsmith_rust::{trace_node, RunType};
use serde_json::json;

async fn llm_node(messages: Vec<String>) -> langsmith_rust::Result<String> {
    // Your node logic here
    let response = call_openai(&messages).await?;
    Ok(response)
}

// Wrap your node function with trace_node
let result = trace_node(
    "llm_node",
    RunType::Llm,
    vec!["Hello".to_string(), "How are you?".to_string()],
    llm_node
).await?;
```

**How it works:**
1. **Before execution**: Serializes function parameters as `inputs` and posts to LangSmith
2. **Executes** the function
3. **After execution**: Serializes return value as `outputs` and patches the run
4. **Error handling**: Automatically captures and traces errors

## Architecture

This crate follows SOLID principles and uses several design patterns:

- **Strategy Pattern** - Different tracing strategies (async/sync)
- **Factory Pattern** - TracerFactory for creating tracers with different configurations
- **Observer Pattern** - Observable nodes for LangGraph integration

See [ARCHITECTURE.md](./ARCHITECTURE.md) for detailed architecture documentation.

## Integration with LangGraph

This crate is designed to integrate seamlessly with LangGraph-style node execution:

```rust
use langsmith_rust::{trace_node, RunType, TracerFactory};
use serde_json::json;

// In your graph node implementation
pub struct GraphNode {
    name: String,
    run_type: RunType,
}

impl GraphNode {
    pub async fn execute(&self, state: State) -> Result<State> {
        trace_node(
            &self.name,
            self.run_type.clone(),
            json!(state),
            |state| async move {
                // Your node logic
                process_state(state).await
            }
        ).await
    }
}
```

See [INTEGRATION.md](./INTEGRATION.md) for detailed integration guide.

## API Reference

### Core Types

#### `Tracer`

Main structure for creating and managing runs.

```rust
// Create a new tracer
let tracer = Tracer::new("Node Name", RunType::Llm, json!({"input": "..."}));

// Configure tracer
let tracer = tracer
    .with_thread_id("thread-123".to_string())
    .with_client(client);

// Create child run
let child = tracer.create_child("Child Node", RunType::Tool, json!({}));

// Post and patch
tracer.post().await?;
tracer.end(json!({"output": "..."}));
tracer.patch().await?;
```

#### `TracerFactory`

Factory for creating tracers with different configurations:

```rust
// Create root tracer
let root = TracerFactory::create_root("Root", RunType::Chain, json!({}));

// Create with thread context
let tracer = TracerFactory::create_with_thread(
    "Node",
    RunType::Llm,
    json!({}),
    "thread-123".to_string()
);

// Create for graph node
let tracer = TracerFactory::create_for_node(
    "Node",
    RunType::Llm,
    json!({}),
    Some(&parent_context)
);
```

### Helper Functions

- `trace_node(name, run_type, inputs, f)` - Wrap async function with tracing
- `trace_node_sync(name, run_type, inputs, f)` - Wrap sync function with tracing

### Run Types

- `RunType::Chain` - Chain execution (orchestrator)
- `RunType::Llm` - LLM call
- `RunType::Tool` - Tool execution
- `RunType::Retriever` - Retrieval operation
- `RunType::Embedding` - Embedding generation
- `RunType::Prompt` - Prompt execution
- `RunType::Runnable` - Generic runnable
- `RunType::Custom(String)` - Custom run type

## Examples

See the `examples/` directory for complete examples:

- `test_llm_tracing.rs` - Basic LLM tracing example
- `decorator_example.rs` - Using trace_node with multiple nodes
- `observable_graph.rs` - Observable graph nodes with Observer pattern

Run examples with:

```bash
cargo run --example test_llm_tracing
```

## Testing

Run all tests:

```bash
cargo test
```

Run specific test suites:

```bash
cargo test --test config_test
cargo test --test models_test
cargo test --test tracer_test
```

## Error Handling

All tracing errors are logged to stderr but **never break your application**. If tracing fails, your code continues to execute normally. This ensures tracing is truly non-intrusive.

```rust
// Even if LangSmith is down, your code continues
let result = trace_node("node", RunType::Llm, input, my_function).await?;
// Your application continues normally
```

## Performance

- **Non-blocking**: All HTTP requests are async and don't block execution
- **Lazy initialization**: Configuration is loaded only when needed
- **Efficient serialization**: Uses `serde_json` for fast serialization
- **Minimal overhead**: Tracing adds <1ms overhead per node

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.

## License

MIT

## Documentation

- [ARCHITECTURE.md](./ARCHITECTURE.md) - Detailed architecture and design patterns
- [INTEGRATION.md](./INTEGRATION.md) - Integration guide for LangGraph
- [docs.md](./docs.md) - Comprehensive documentation for LLMs
- [READING_GUIDE.md](./READING_GUIDE.md) - Guide to understanding the codebase
