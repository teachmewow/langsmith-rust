# Integration Guide: LangSmith Rust with LangGraph

This guide explains how to integrate `langsmith-rust` with your LangGraph-style node execution system.

## Overview

The crate is designed to work seamlessly with graph-based agent architectures where:
- Nodes execute functions
- Nodes can have parent-child relationships
- State flows between nodes
- Errors need to be traced

## Basic Integration Pattern

### 1. Node Structure

Your graph nodes should follow this pattern:

```rust
use langsmith_rust::{trace_node, RunType, TracerFactory, TraceContext};
use serde_json::Value;

pub struct GraphNode {
    pub name: String,
    pub run_type: RunType,
    pub parent_context: Option<TraceContext>,
}

impl GraphNode {
    pub async fn execute(&self, state: State) -> Result<State, NodeError> {
        // Wrap execution with tracing
        trace_node(
            &self.name,
            self.run_type.clone(),
            json!(state),  // Serialize state as inputs
            |state| async move {
                // Your actual node logic here
                self.process_state(state).await
            }
        ).await
        .map_err(|e| NodeError::TracingError(e.to_string()))
    }
    
    async fn process_state(&self, state: State) -> Result<State, NodeError> {
        // Your business logic
        Ok(state)
    }
}
```

### 2. Graph Execution

```rust
use langsmith_rust::{TracerFactory, RunType, TraceContext};
use serde_json::json;

pub struct Graph {
    nodes: Vec<GraphNode>,
    thread_id: String,
}

impl Graph {
    pub async fn execute(&self, initial_state: State) -> Result<State, GraphError> {
        // Create root tracer context
        let root_tracer = TracerFactory::create_root(
            "Graph Execution",
            RunType::Chain,
            json!(initial_state)
        ).with_thread_id(self.thread_id.clone());
        
        let root_context = root_tracer.context();
        
        // Post root run
        root_tracer.post().await?;
        
        // Execute nodes with context propagation
        let mut current_state = initial_state;
        let mut current_context = root_context;
        
        for node in &self.nodes {
            // Create node with parent context
            let node_tracer = TracerFactory::create_for_node(
                &node.name,
                node.run_type.clone(),
                json!(current_state),
                Some(&current_context)
            );
            
            // Execute node (trace_node handles tracing internally)
            current_state = trace_node(
                &node.name,
                node.run_type.clone(),
                json!(current_state),
                |state| async move {
                    node.execute(state).await
                }
            ).await?;
            
            // Update context for next node
            current_context = node_tracer.context();
        }
        
        // End root run
        let mut root_tracer = TracerFactory::create_root(
            "Graph Execution",
            RunType::Chain,
            json!(initial_state)
        );
        root_tracer.end(json!(current_state));
        root_tracer.patch().await?;
        
        Ok(current_state)
    }
}
```

### 3. Node Types

Different node types map to different `RunType` values:

```rust
pub enum NodeType {
    LLM,      // → RunType::Llm
    Tool,     // → RunType::Tool
    Chain,    // → RunType::Chain
    Retriever,// → RunType::Retriever
    Custom(String), // → RunType::Custom(String)
}

impl From<NodeType> for RunType {
    fn from(node_type: NodeType) -> Self {
        match node_type {
            NodeType::LLM => RunType::Llm,
            NodeType::Tool => RunType::Tool,
            NodeType::Chain => RunType::Chain,
            NodeType::Retriever => RunType::Retriever,
            NodeType::Custom(s) => RunType::Custom(s),
        }
    }
}
```

## Advanced Patterns

### 1. Observable Nodes

Use the Observer pattern for additional observability:

```rust
use langsmith_rust::observability::{ObservableNodeWrapper, Observer, LangSmithObserver};
use std::sync::Arc;

pub struct ObservableGraphNode {
    wrapper: ObservableNodeWrapper,
    // ... your node data
}

impl ObservableGraphNode {
    pub fn new(name: String, run_type: RunType) -> Self {
        let observer: Arc<dyn Observer> = Arc::new(LangSmithObserver::new());
        
        Self {
            wrapper: ObservableNodeWrapper::new(name, run_type)
                .with_observer(observer),
            // ...
        }
    }
    
    pub async fn execute(&self, state: State) -> Result<State> {
        self.wrapper.execute(
            json!(state),
            |state| async move {
                // Your logic
                process_state(state).await
            }
        ).await
    }
}
```

### 2. Conditional Tracing

Only trace when enabled:

```rust
use langsmith_rust::Config;

pub async fn execute_node(node: &GraphNode, state: State) -> Result<State> {
    if Config::is_tracing_enabled() {
        trace_node(&node.name, node.run_type.clone(), json!(state), |state| async move {
            node.process(state).await
        }).await
    } else {
        node.process(state).await
    }
}
```

### 3. Error Tracing

Errors are automatically traced:

```rust
pub async fn risky_node(state: State) -> Result<State> {
    trace_node(
        "risky_node",
        RunType::Tool,
        json!(state),
        |state| async move {
            // If this fails, error is automatically traced
            dangerous_operation(state).await?
        }
    ).await
    // Error is already traced, just propagate
}
```

### 4. Metrics Tracking

Add metrics to your runs:

```rust
use langsmith_rust::models::Metrics;

pub async fn llm_node(messages: Vec<String>) -> Result<String> {
    let mut tracer = Tracer::new("llm_node", RunType::Llm, json!({"messages": messages}));
    tracer.post().await?;
    
    let (response, usage) = call_openai(&messages).await?;
    
    // Add metrics
    tracer.run.prompt_tokens = Some(usage.prompt_tokens);
    tracer.run.completion_tokens = Some(usage.completion_tokens);
    tracer.run.total_tokens = Some(usage.total_tokens);
    tracer.run.total_cost = Some(usage.cost);
    
    tracer.end(json!({"completion": response}));
    tracer.patch().await?;
    
    Ok(response)
}
```

## State Serialization

Your `State` type must implement `Serialize`:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub messages: Vec<Message>,
    pub metadata: HashMap<String, Value>,
    // ...
}
```

**Important**: The crate automatically wraps primitive types in objects:
- `String` → `{"input": "..."}`
- `i32` → `{"input": 42}`
- `State` → `{"input": {...}}` (if State is an object, it's used as-is)

## Thread Management

For multi-threaded/concurrent execution:

```rust
use langsmith_rust::{TracerFactory, TraceContext};
use uuid::Uuid;

pub struct ThreadManager {
    thread_id: String,
    trace_context: TraceContext,
}

impl ThreadManager {
    pub fn new() -> Self {
        Self {
            thread_id: Uuid::new_v4().to_string(),
            trace_context: TraceContext::new(Uuid::new_v4())
                .with_thread_id(Uuid::new_v4().to_string()),
        }
    }
    
    pub fn create_node_tracer(&self, name: &str, run_type: RunType, inputs: Value) -> Tracer {
        TracerFactory::create_for_node(
            name,
            run_type,
            inputs,
            Some(&self.trace_context)
        )
    }
}
```

## Best Practices

1. **Always use `trace_node`** for automatic tracing
2. **Use `TracerFactory`** for consistent tracer creation
3. **Propagate context** through parent-child relationships
4. **Handle errors gracefully** - tracing errors shouldn't break your app
5. **Use appropriate RunType** - helps with filtering in LangSmith UI
6. **Set thread_id** for conversation tracking
7. **Add metrics** when available (tokens, costs)

## Example: Complete Graph Node

```rust
use langsmith_rust::{trace_node, RunType, TracerFactory, TraceContext};
use serde_json::json;

pub struct LLMNode {
    name: String,
    model: String,
    temperature: f64,
}

impl LLMNode {
    pub fn new(name: String, model: String) -> Self {
        Self {
            name,
            model,
            temperature: 0.7,
        }
    }
    
    pub async fn execute(
        &self,
        state: State,
        parent_context: Option<&TraceContext>
    ) -> Result<State> {
        trace_node(
            &self.name,
            RunType::Llm,
            json!({
                "messages": state.messages,
                "model": self.model,
                "temperature": self.temperature
            }),
            |inputs| async move {
                // Deserialize inputs
                let messages: Vec<Message> = inputs["messages"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|m| serde_json::from_value(m.clone()).unwrap())
                    .collect();
                
                // Call LLM
                let response = call_openai(&messages, &self.model).await?;
                
                // Update state
                let mut new_state = state;
                new_state.messages.push(response);
                
                Ok(new_state)
            }
        ).await
    }
}
```

## Troubleshooting

### Tracing not appearing in LangSmith

1. Check `LANGSMITH_TRACING=true` in `.env`
2. Verify `LANGSMITH_API_KEY` is set
3. Check network connectivity
4. Look for errors in stderr

### Parent-child relationships not showing

1. Ensure you're using `create_child()` or `create_for_node()` with context
2. Verify `trace_id` is propagated correctly
3. Check `dotted_order` is generated correctly

### Performance issues

1. Tracing is async and non-blocking
2. If still slow, check network latency to LangSmith
3. Consider batching (future feature)

