# Architecture Documentation

## Overview

This crate implements a modular, extensible architecture for tracing to LangSmith. It follows SOLID principles and uses several design patterns to ensure maintainability and scalability.

## Hierarchical Structure

```
langsmith-rust/
├── src/
│   ├── lib.rs                    # Main entry point, re-exports
│   ├── config/                   # Configuration management
│   │   ├── mod.rs
│   │   └── env.rs                # Environment variable loading
│   ├── client/                   # HTTP client layer
│   │   ├── mod.rs
│   │   └── http.rs               # LangSmith API client
│   ├── models/                   # Data models
│   │   ├── mod.rs
│   │   ├── run.rs                # Run, RunType, RunUpdate
│   │   ├── messages.rs           # Message types (AI, Tool, Human)
│   │   └── metrics.rs            # Metrics (tokens, costs)
│   ├── tracing/                  # Core tracing logic
│   │   ├── mod.rs
│   │   ├── tracer.rs             # Tracer (main tracing struct)
│   │   ├── context.rs            # TraceContext (propagation)
│   │   └── decorator.rs          # trace_node helpers
│   ├── strategies/               # Strategy pattern implementations
│   │   ├── mod.rs
│   │   ├── tracing_strategy.rs   # Tracing strategies (async/sync)
│   │   └── serialization_strategy.rs  # Serialization strategies
│   ├── factories/                # Factory pattern
│   │   ├── mod.rs
│   │   └── tracer_factory.rs     # TracerFactory
│   ├── observability/            # Observer pattern
│   │   ├── mod.rs
│   │   ├── observer.rs           # Observer trait
│   │   ├── observable.rs         # Observable trait
│   │   └── node_wrapper.rs       # ObservableNodeWrapper
│   ├── utils/                    # Utilities
│   │   ├── mod.rs
│   │   ├── serialization.rs      # JSON serialization helpers
│   │   └── validation.rs         # Data validation
│   └── error.rs                  # Error types
```

## Class Relationships

### Core Classes

```
┌─────────────────┐
│     Config      │
│  (Singleton)    │
└────────┬────────┘
         │ provides config
         │
         ▼
┌─────────────────┐      ┌─────────────────┐
│ LangSmithClient │◄─────│     Tracer      │
│                 │ uses │                 │
└─────────────────┘      └────────┬────────┘
                                  │
                                  │ creates
                                  ▼
                          ┌─────────────────┐
                          │       Run        │
                          │  (Data Model)    │
                          └─────────────────┘
```

### Tracing Flow

```
User Code
    │
    ├─► TracerFactory.create() ──┐
    │                              │
    ├─► Tracer::new() ────────────┼──► Tracer
    │                              │      │
    └─► trace_node() ──────────────┘      │
                                         │
                                         ├─► Run (created)
                                         │
                                         ├─► LangSmithClient.post_run()
                                         │      │
                                         │      └─► HTTP POST /runs
                                         │
                                         ├─► Function execution
                                         │
                                         ├─► Run.end(outputs)
                                         │
                                         └─► LangSmithClient.patch_run()
                                                │
                                                └─► HTTP PATCH /runs/{id}
```

### Parent-Child Relationship

```
Root Tracer
    │
    ├─► trace_id: UUID-1
    ├─► dotted_order: "20240101T120000Zuuid-1"
    │
    └─► create_child()
            │
            ├─► parent_run_id: UUID-1
            ├─► trace_id: UUID-1 (inherited)
            └─► dotted_order: "20240101T120000Zuuid-1.20240101T120100Zuuid-2"
                    │
                    └─► create_child()
                            │
                            ├─► parent_run_id: UUID-2
                            ├─► trace_id: UUID-1 (inherited)
                            └─► dotted_order: "...uuid-1....uuid-2....uuid-3"
```

## Data Flow

### 1. Initialization Flow

```
Application Start
    │
    ├─► langsmith_rust::init()
    │      │
    │      └─► dotenvy::dotenv() ──► Load .env file
    │
    └─► Config::get() ──► Config::from_env()
            │
            ├─► Read LANGSMITH_TRACING
            ├─► Read LANGSMITH_ENDPOINT
            ├─► Read LANGSMITH_API_KEY
            └─► Read LANGSMITH_PROJECT
```

### 2. Tracing Flow (trace_node)

```
trace_node("node_name", RunType::Llm, inputs, function)
    │
    ├─► Check Config::is_tracing_enabled()
    │      │
    │      └─► If false: Execute function directly, return
    │
    ├─► ensure_inputs_object(inputs) ──► Convert to JSON object
    │      │
    │      └─► If primitive: wrap in {"input": value}
    │
    ├─► Tracer::new("node_name", RunType::Llm, inputs_json)
    │      │
    │      ├─► Run::new() ──► Generate UUID, timestamp
    │      └─► Set session_name from Config
    │
    ├─► tracer.post() ──► POST /runs
    │      │
    │      ├─► Initialize trace_id if root
    │      ├─► Generate dotted_order
    │      └─► LangSmithClient.post_run()
    │             │
    │             └─► HTTP POST to LangSmith API
    │
    ├─► Execute function(inputs)
    │      │
    │      ├─► Success ──┐
    │      │              │
    │      └─► Error ─────┼──► tracer.set_error()
    │                     │
    ├─► ensure_outputs_object(output) ──► Convert to JSON object
    │      │
    │      └─► If primitive: wrap in {"output": value}
    │
    ├─► tracer.end(outputs_json)
    │      │
    │      └─► Run.end() ──► Set outputs, end_time
    │
    └─► tracer.patch() ──► PATCH /runs/{id}
            │
            ├─► RunUpdate::from(&run)
            └─► LangSmithClient.patch_run()
                   │
                   └─► HTTP PATCH to LangSmith API
```

### 3. Hierarchical Tracing Flow

```
Root Node (Chain)
    │
    ├─► Tracer::new("root", RunType::Chain, inputs)
    │      │
    │      ├─► trace_id = run.id (UUID-1)
    │      └─► dotted_order = "20240101T120000Zuuid-1"
    │
    ├─► tracer.post() ──► POST /runs (root run)
    │
    └─► Child Node (LLM)
            │
            ├─► tracer.create_child("llm", RunType::Llm, inputs)
            │      │
            │      ├─► parent_run_id = parent.run.id
            │      ├─► trace_id = parent.trace_id (UUID-1)
            │      └─► dotted_order = parent + ".20240101T120100Zuuid-2"
            │
            ├─► child.post() ──► POST /runs (child run)
            │
            ├─► Execute LLM call
            │
            ├─► child.end(outputs)
            │
            └─► child.patch() ──► PATCH /runs/{child_id}
```

## Design Patterns

### 1. Strategy Pattern

**Purpose**: Allow different implementations of tracing and serialization

**Implementation**:
- `TracingStrategy` trait - defines interface for tracing operations
- `AsyncTracingStrategy` - async implementation
- `SyncTracingStrategy` - sync implementation (uses blocking runtime)
- `SerializationStrategy` trait - defines interface for serialization
- `DefaultSerializationStrategy` - wraps primitives in objects

**Benefits**:
- Easy to add new strategies (e.g., BatchTracingStrategy)
- Testable with mock strategies
- Runtime selection of strategy

### 2. Factory Pattern

**Purpose**: Centralize creation of complex objects (Tracers)

**Implementation**:
- `TracerFactory` - static factory methods
- Methods: `create()`, `create_with_thread()`, `create_root()`, `create_for_node()`

**Benefits**:
- Consistent tracer creation
- Hides complexity of configuration
- Easy to extend with new creation patterns

### 3. Observer Pattern

**Purpose**: Allow nodes to be observed for events (start, end, error)

**Implementation**:
- `Observer` trait - defines observer interface
- `Observable` trait - defines observable interface
- `ObservableNodeWrapper` - wraps nodes to make them observable
- `LangSmithObserver` - concrete observer that traces to LangSmith

**Benefits**:
- Decouples tracing from node execution
- Multiple observers can be attached
- Easy to add logging, metrics, etc.

## Module Responsibilities

### config/
- **Responsibility**: Load and manage configuration from environment
- **Key Types**: `Config`
- **Pattern**: Singleton (using `Lazy<Mutex<Option<Config>>>`)

### client/
- **Responsibility**: HTTP communication with LangSmith API
- **Key Types**: `LangSmithClient`
- **Methods**: `post_run()`, `patch_run()`

### models/
- **Responsibility**: Data structures representing LangSmith entities
- **Key Types**: `Run`, `RunType`, `RunUpdate`, `Message`, `Metrics`
- **Note**: Pure data structures, no business logic

### tracing/
- **Responsibility**: Core tracing logic and context propagation
- **Key Types**: `Tracer`, `TraceContext`
- **Key Functions**: `trace_node()`, `trace_node_sync()`

### strategies/
- **Responsibility**: Strategy pattern implementations
- **Key Types**: `TracingStrategy`, `SerializationStrategy`

### factories/
- **Responsibility**: Factory pattern for tracer creation
- **Key Types**: `TracerFactory`

### observability/
- **Responsibility**: Observer pattern for node observation
- **Key Types**: `Observer`, `Observable`, `ObservableNodeWrapper`

### utils/
- **Responsibility**: Utility functions (serialization, validation)
- **Key Functions**: `ensure_object()`, `validate_run()`

## Error Handling

All errors are wrapped in `LangSmithError` enum:

```rust
pub enum LangSmithError {
    Http(reqwest::Error),
    Serialization(serde_json::Error),
    Config(String),
    TracingDisabled,
    Other(String),
}
```

**Error Propagation Strategy**:
- Tracing errors are logged but **never break application execution**
- Functions return `Result<T>` but tracing failures are caught and logged
- Application code continues even if LangSmith is down

## Thread Safety

- `Config`: Thread-safe singleton using `Lazy<Mutex<Option<Config>>>`
- `Tracer`: Not thread-safe (should be used within single async task)
- `LangSmithClient`: Thread-safe (uses `Arc` internally)
- `Observer`: Thread-safe (uses `Arc<dyn Observer>`)

## Performance Considerations

1. **Lazy Initialization**: Config is loaded only when needed
2. **Non-blocking**: All HTTP calls are async
3. **Efficient Serialization**: Uses `serde_json` (fast)
4. **Minimal Allocations**: Reuses clients where possible
5. **Early Returns**: Checks `is_tracing_enabled()` before any work

## Extension Points

### Adding a New Run Type

```rust
// In models/run.rs
pub enum RunType {
    // ... existing variants
    Custom(String),
    MyNewType,  // Add here
}
```

### Adding a New Strategy

```rust
// In strategies/
pub struct MyNewStrategy;

impl TracingStrategy for MyNewStrategy {
    // Implement trait methods
}
```

### Adding a New Observer

```rust
pub struct MyObserver;

impl Observer for MyObserver {
    fn on_node_start(&self, node_name: &str, inputs: &Value) {
        // Your logic
    }
    // ... implement other methods
}
```

