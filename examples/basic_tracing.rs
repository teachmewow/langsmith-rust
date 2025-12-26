// Example: Basic tracing usage
use langsmith_rust::{trace_node, RunType, Tracer, Result};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize (loads .env automatically)
    langsmith_rust::init();

    // Example 1: Manual tracing
    println!("Example 1: Manual tracing");
    let mut tracer = Tracer::new(
        "Agent Pipeline",
        RunType::Chain,
        json!({"question": "What is the weather?"}),
    )
    .with_thread_id("thread-123".to_string());

    tracer.post().await?;

    // Create child run for LLM
    let mut llm_run = tracer.create_child(
        "OpenAI Call",
        RunType::Llm,
        json!({"messages": [{"role": "user", "content": "What is the weather?"}]}),
    );
    llm_run.post().await?;

    // Simulate LLM call
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Update with outputs
    llm_run.end(json!({"completion": "The weather is sunny!"}));
    llm_run.patch().await?;

    // End parent run
    tracer.end(json!({"answer": "The weather is sunny!"}));
    tracer.patch().await?;

    println!("Manual tracing completed!");

    // Example 2: Using trace_node helper
    println!("\nExample 2: Using trace_node helper");
    
    async fn my_node(input: serde_json::Value) -> Result<serde_json::Value> {
        // Simulate some work
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        Ok(json!({"result": "processed", "input": input}))
    }

    let result = trace_node(
        "my_custom_node",
        RunType::Runnable,
        json!({"data": "test"}),
        my_node,
    )
    .await?;

    println!("Result: {}", result);
    println!("Automatic tracing completed!");

    Ok(())
}

