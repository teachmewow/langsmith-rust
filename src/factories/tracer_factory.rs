use crate::client::LangSmithClient;
use crate::models::run::RunType;
use crate::tracing::tracer::Tracer;
use crate::tracing::context::TraceContext;
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

/// Factory for creating Tracer instances with different configurations
pub struct TracerFactory;

impl TracerFactory {
    /// Create a basic tracer
    pub fn create(name: impl Into<String>, run_type: RunType, inputs: Value) -> Tracer {
        Tracer::new(name, run_type, inputs)
    }

    /// Create a tracer with a specific client
    pub fn create_with_client(
        name: impl Into<String>,
        run_type: RunType,
        inputs: Value,
        client: Arc<LangSmithClient>,
    ) -> Tracer {
        Tracer::new(name, run_type, inputs).with_client(client)
    }

    /// Create a tracer with thread context
    pub fn create_with_thread(
        name: impl Into<String>,
        run_type: RunType,
        inputs: Value,
        thread_id: String,
    ) -> Tracer {
        Tracer::new(name, run_type, inputs).with_thread_id(thread_id)
    }

    /// Create a tracer with trace context
    pub fn create_with_context(
        name: impl Into<String>,
        run_type: RunType,
        inputs: Value,
        context: &TraceContext,
    ) -> Tracer {
        Tracer::new(name, run_type, inputs).with_context(context)
    }

    /// Create a root tracer (for starting a new trace)
    pub fn create_root(
        name: impl Into<String>,
        run_type: RunType,
        inputs: Value,
    ) -> Tracer {
        let mut tracer = Tracer::new(name, run_type, inputs);
        // Ensure trace_id is set - need to post first to initialize
        // For now, we'll create a context manually
        use uuid::Uuid;
        let trace_id = Uuid::new_v4();
        let context = TraceContext::new(trace_id);
        tracer = tracer.with_context(&context);
        tracer
    }

    /// Create a tracer for a graph node
    pub fn create_for_node(
        node_name: impl Into<String>,
        run_type: RunType,
        inputs: Value,
        parent_context: Option<&TraceContext>,
    ) -> Tracer {
        let mut tracer = Tracer::new(node_name, run_type, inputs);
        
        if let Some(context) = parent_context {
            tracer = tracer.with_context(context);
        } else {
            // Create new root context
            let trace_id = Uuid::new_v4();
            let context = TraceContext::new(trace_id);
            tracer = tracer.with_context(&context);
        }
        
        tracer
    }
}

