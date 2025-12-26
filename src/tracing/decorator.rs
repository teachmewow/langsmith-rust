use crate::error::Result;
use crate::models::run::RunType;
use crate::tracing::tracer::Tracer;
use crate::utils::serialization::{ensure_inputs_object, ensure_outputs_object};
use serde::Serialize;
use std::future::Future;

/// Helper function to trace a node execution
/// 
/// This function wraps a node execution with tracing:
/// 1. Creates a tracer with serialized inputs
/// 2. Posts the run to LangSmith (start_time, inputs)
/// 3. Executes the function
/// 4. Updates the run with outputs and end_time
/// 5. Handles errors appropriately
pub async fn trace_node<F, Fut, I, O>(
    name: &str,
    run_type: RunType,
    inputs: I,
    f: F,
) -> Result<O>
where
    F: FnOnce(I) -> Fut,
    Fut: Future<Output = Result<O>>,
    I: Serialize,
    O: Serialize,
{
    // Check if tracing is enabled
    if !crate::config::Config::is_tracing_enabled() {
        return f(inputs).await;
    }

    // 1. Serialize inputs - ensure it's always an object
    let inputs_value = ensure_inputs_object(&inputs)
        .map_err(|e| crate::error::LangSmithError::Serialization(e))?;

    // 2. Create tracer
    let mut tracer = Tracer::new(name, run_type, inputs_value);

    // 3. POST /runs - save initial run (start_time, inputs)
    if let Err(e) = tracer.post().await {
        // Log error but don't fail the node execution
        eprintln!("LangSmith tracing error (post): {}", e);
    }

    // 4. Execute the function
    match f(inputs).await {
        Ok(output) => {
            // 5. Serialize outputs - ensure it's always an object
            let output_value = ensure_outputs_object(&output)
                .map_err(|e| crate::error::LangSmithError::Serialization(e))?;

            // 6. Mark run as finished and PATCH /runs/{run_id} - save outputs and end_time
            tracer.end(output_value);
            if let Err(e) = tracer.patch().await {
                // Log error but don't fail the node execution
                eprintln!("LangSmith tracing error (patch): {}", e);
            }

            Ok(output)
        }
        Err(e) => {
            // In case of error, mark run with error
            tracer.set_error(&e.to_string());
            if let Err(trace_err) = tracer.patch().await {
                eprintln!("LangSmith tracing error (patch): {}", trace_err);
            }
            Err(e)
        }
    }
}

/// Synchronous version of trace_node
pub fn trace_node_sync<F, I, O>(
    name: &str,
    run_type: RunType,
    inputs: I,
    f: F,
) -> Result<O>
where
    F: FnOnce(I) -> Result<O>,
    I: Serialize,
    O: Serialize,
{
    // Check if tracing is enabled
    if !crate::config::Config::is_tracing_enabled() {
        return f(inputs);
    }

    // 1. Serialize inputs - ensure it's always an object
    let inputs_value = ensure_inputs_object(&inputs)
        .map_err(|e| crate::error::LangSmithError::Serialization(e))?;

    // 2. Create tracer
    let mut tracer = Tracer::new(name, run_type, inputs_value);

    // 3. POST /runs - save initial run (start_time, inputs)
    // For sync version, we need to use tokio runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    if let Err(e) = rt.block_on(tracer.post()) {
        eprintln!("LangSmith tracing error (post): {}", e);
    }

    // 4. Execute the function
    match f(inputs) {
        Ok(output) => {
            // 5. Serialize outputs - ensure it's always an object
            let output_value = ensure_outputs_object(&output)
                .map_err(|e| crate::error::LangSmithError::Serialization(e))?;

            // 6. Mark run as finished and PATCH /runs/{run_id} - save outputs and end_time
            tracer.end(output_value);
            if let Err(e) = rt.block_on(tracer.patch()) {
                eprintln!("LangSmith tracing error (patch): {}", e);
            }

            Ok(output)
        }
        Err(e) => {
            // In case of error, mark run with error
            tracer.set_error(&e.to_string());
            if let Err(trace_err) = rt.block_on(tracer.patch()) {
                eprintln!("LangSmith tracing error (patch): {}", trace_err);
            }
            Err(e)
        }
    }
}

