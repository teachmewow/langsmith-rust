# LangSmith Rust - Comprehensive Documentation

## Purpose

This crate provides a Rust implementation for manual tracing to LangSmith, enabling observability for AI agent systems built with Rust. It is designed to integrate seamlessly with LangGraph-style architectures where nodes execute functions and state flows through a graph.

## Core Concepts

### Tracing

Tracing is the process of recording execution metadata (inputs, outputs, timing, errors) and sending it to LangSmith for visualization and analysis. Each execution unit (node, function, operation) becomes a "run" in LangSmith.

### Runs

A "run" represents a single execution unit:
- **Root Run**: The top-level execution (e.g., entire graph execution)
- **Child Run**: A nested execution (e.g., LLM call within a chain)
- **Run Hierarchy**: Parent-child relationships form a tree structure

### Context Propagation

Context (trace_id, parent_run_id, dotted_order) flows from parent to child, maintaining the execution hierarchy. This allows LangSmith to visualize the complete execution tree.

## Architecture Overview

The crate follows a modular architecture with clear separation of concerns:

1. **Configuration Layer** (`config/`): Loads settings from environment variables
2. **Client Layer** (`client/`): Handles HTTP communication with LangSmith API
3. **Model Layer** (`models/`): Defines data structures (Run, Message, Metrics)
4. **Tracing Layer** (`tracing/`): Core tracing logic and context management
5. **Strategy Layer** (`strategies/`): Pluggable strategies for tracing and serialization
6. **Factory Layer** (`factories/`): Factory methods for creating tracers
7. **Observability Layer** (`observability/`): Observer pattern for node observation
8. **Utility Layer** (`utils/`): Helper functions for serialization and validation

## Key Components

### Tracer

The `Tracer` struct is the main interface for tracing. It wraps a `Run` and provides methods to:
- Create child tracers
- Post runs to LangSmith
- Update runs with outputs
- Handle errors

**Creation**:
```rust
let tracer = Tracer::new("node_name", RunType::Llm, json!({"input": "..."}));
```

**Usage**:
```rust
tracer.post().await?;  // Send initial run
// ... execute function ...
tracer.end(json!({"output": "..."}));  // Mark as finished
tracer.patch().await?;  // Send updates
```

### trace_node Helper

The `trace_node` function automatically wraps a function with tracing:

```rust
let result = trace_node(
    "function_name",
    RunType::Llm,
    input_data,
    |input| async move {
        // Your function logic
        process(input).await
    }
).await?;
```

This automatically:
1. Creates a tracer
2. Posts the run (with inputs)
3. Executes the function
4. Updates the run (with outputs or error)

### TracerFactory

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

// Create for graph node with parent context
let tracer = TracerFactory::create_for_node(
    "Node",
    RunType::Llm,
    json!({}),
    Some(&parent_context)
);
```

## Data Flow

### Initialization

1. Application calls `langsmith_rust::init()`
2. Loads `.env` file (if exists)
3. Reads environment variables:
   - `LANGSMITH_TRACING` (enables/disables tracing)
   - `LANGSMITH_ENDPOINT` (API endpoint)
   - `LANGSMITH_API_KEY` (authentication)
   - `LANGSMITH_PROJECT` (project name)
   - `LANGSMITH_TENANT_ID` (optional workspace ID)

### Tracing Execution

1. **Create Tracer**: User creates a `Tracer` with name, type, and inputs
2. **Post Run**: Tracer posts initial run to LangSmith (POST /runs)
   - Generates UUID for run_id
   - Sets trace_id (same as run_id for root)
   - Generates dotted_order for ordering
   - Sets start_time
3. **Execute Function**: User's function executes
4. **Update Run**: Tracer updates run with outputs (PATCH /runs/{id})
   - Sets outputs
   - Sets end_time
   - Sets error (if any)

### Hierarchical Tracing

When creating child runs:
1. Child inherits `trace_id` from parent
2. Child sets `parent_run_id` to parent's `run_id`
3. Child generates `dotted_order` by appending to parent's `dotted_order`
4. This creates a tree structure in LangSmith

## Run Types

Different execution types map to different `RunType` values:

- `Chain`: Orchestrator/coordinator nodes
- `Llm`: LLM API calls
- `Tool`: Tool/function executions
- `Retriever`: Retrieval operations
- `Embedding`: Embedding generation
- `Prompt`: Prompt execution
- `Runnable`: Generic runnable
- `Custom(String)`: Custom types

## Message Types

The crate supports LangChain-compatible message types:

- `AIMessage`: AI responses (may include tool_calls)
- `ToolMessage`: Tool execution results
- `HumanMessage`: Human/user inputs
- `SystemMessage`: System prompts

These types ensure compatibility with LangChain's data model.

## Serialization

All data sent to LangSmith must be JSON objects. The crate automatically:
- Wraps primitive types (String, i32, bool) in objects: `{"input": value}`
- Preserves objects as-is
- Ensures `inputs` and `outputs` are always objects

## Error Handling

Tracing errors are **non-fatal**:
- Errors are logged to stderr
- Application execution continues normally
- Tracing failures don't break your code

This ensures tracing is truly non-intrusive.

## Thread Safety

- `Config`: Thread-safe singleton
- `Tracer`: Not thread-safe (use within single async task)
- `LangSmithClient`: Thread-safe (uses Arc internally)
- `Observer`: Thread-safe (uses Arc<dyn Observer>)

## Performance

- **Non-blocking**: All HTTP calls are async
- **Lazy initialization**: Config loaded only when needed
- **Efficient serialization**: Uses serde_json
- **Minimal overhead**: <1ms per node

## Design Patterns Used

1. **Strategy Pattern**: Pluggable tracing and serialization strategies
2. **Factory Pattern**: Centralized tracer creation
3. **Observer Pattern**: Observable nodes for additional observability
4. **Singleton Pattern**: Global configuration access

## Integration Points

### With LangGraph-style Systems

1. Wrap node execution with `trace_node`
2. Use `TracerFactory` to create tracers with context
3. Propagate `TraceContext` through graph execution
4. Use appropriate `RunType` for each node type

### With Async Runtimes

Works with any async runtime (Tokio, async-std, etc.) via async/await.

### With Serialization

Requires `Serialize` trait for inputs/outputs. Works with:
- serde_json::Value
- Custom structs implementing Serialize
- Primitives (automatically wrapped)

## Configuration

All configuration via environment variables:

```bash
LANGSMITH_TRACING=true              # Enable/disable tracing
LANGSMITH_ENDPOINT=https://...     # API endpoint
LANGSMITH_API_KEY=sk-...           # API key (required)
LANGSMITH_PROJECT=my-project       # Project name
LANGSMITH_TENANT_ID=...            # Optional workspace ID
```

## API Endpoints

The crate interacts with LangSmith API:

- `POST /runs` - Create a new run
- `PATCH /runs/{run_id}` - Update an existing run

## Run Data Structure

A run contains:
- `id`: Unique identifier (UUID)
- `name`: Human-readable name
- `run_type`: Type of run (Llm, Tool, etc.)
- `inputs`: Input data (JSON object)
- `outputs`: Output data (JSON object, optional)
- `start_time`: When execution started
- `end_time`: When execution ended (optional)
- `trace_id`: Root trace identifier
- `parent_run_id`: Parent run identifier (optional)
- `dotted_order`: Ordering string for hierarchy
- `thread_id`: Conversation/thread identifier
- `session_name`: Session/project name
- `error`: Error message (optional)
- `tags`: Tags for filtering
- `extra`: Additional metadata
- Metrics: `prompt_tokens`, `completion_tokens`, `total_tokens`, costs

## Usage Patterns

### Pattern 1: Simple Function Tracing

```rust
let result = trace_node("my_function", RunType::Runnable, input, my_function).await?;
```

### Pattern 2: Manual Tracing

```rust
let mut tracer = Tracer::new("node", RunType::Llm, json!({"input": "..."}));
tracer.post().await?;
let output = execute_function().await?;
tracer.end(json!({"output": output}));
tracer.patch().await?;
```

### Pattern 3: Hierarchical Tracing

```rust
let mut parent = Tracer::new("parent", RunType::Chain, json!({}));
parent.post().await?;

let mut child = parent.create_child("child", RunType::Llm, json!({}));
child.post().await?;
// ... execute ...
child.end(json!({}));
child.patch().await?;

parent.end(json!({}));
parent.patch().await?;
```

### Pattern 4: Graph Node Integration

```rust
pub async fn execute_node(node: &Node, state: State) -> Result<State> {
    trace_node(
        &node.name,
        node.run_type.clone(),
        json!(state),
        |state| async move {
            node.process(state).await
        }
    ).await
}
```

## Extension Points

### Adding Custom Run Types

Extend `RunType` enum in `models/run.rs`.

### Adding Custom Strategies

Implement `TracingStrategy` or `SerializationStrategy` traits.

### Adding Custom Observers

Implement `Observer` trait and attach to `ObservableNodeWrapper`.

## Testing

The crate includes comprehensive tests:
- Unit tests for each module
- Integration tests for end-to-end flows
- Tests can run with tracing disabled (set `LANGSMITH_TRACING=false`)

## Limitations

1. No batching yet (future feature)
2. No retry logic (future feature)
3. Requires async runtime
4. Inputs/outputs must be serializable

## Future Enhancements

- Batch tracing for better performance
- Retry logic for failed requests
- Metrics aggregation
- Custom transport layers
- Synchronous tracing without runtime

## See Also

- `ARCHITECTURE.md` - Detailed architecture documentation
- `INTEGRATION.md` - Integration guide for LangGraph
- `READING_GUIDE.md` - Guide to understanding the codebase
- Examples in `examples/` directory

