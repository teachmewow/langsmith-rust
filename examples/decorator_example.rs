// Example: Using decorator pattern for tracing nodes
use langsmith_rust::{trace_node, RunType, Result};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize (loads .env automatically)
    langsmith_rust::init();

    println!("Example: Using trace_node decorator with multiple nodes\n");

    // Example 1: LLM Node
    async fn llm_node(messages: Vec<String>) -> Result<String> {
        // Simulate LLM call
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        Ok(format!("Response to: {}", messages.join(", ")))
    }

    let messages = vec!["Hello".to_string(), "How are you?".to_string()];
    let llm_result = trace_node(
        "llm_node",
        RunType::Llm,
        messages,
        llm_node,
    ).await?;

    println!("LLM Result: {}\n", llm_result);

    // Example 2: Tool Node
    async fn search_tool(_query: String) -> Result<serde_json::Value> {
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        Ok(json!({
            "results": [
                {"title": "Result 1", "url": "https://example.com/1"},
                {"title": "Result 2", "url": "https://example.com/2"}
            ]
        }))
    }

    let search_result = trace_node(
        "search_tool",
        RunType::Tool,
        "Rust programming".to_string(),
        search_tool,
    ).await?;

    println!("Search Result: {}\n", search_result);

    // Example 3: Chain Node (orchestrates other nodes)
    async fn agent_chain(user_input: String) -> Result<String> {
        // Step 1: Search
        let search_query = format!("Search: {}", user_input);
        let search_results = trace_node(
            "search_step",
            RunType::Tool,
            search_query.clone(),
            search_tool,
        ).await?;

        // Step 2: LLM with search results
        let messages = vec![
            format!("User asked: {}", user_input),
            format!("Search results: {}", search_results),
        ];
        let llm_response = trace_node(
            "llm_with_context",
            RunType::Llm,
            messages,
            llm_node,
        ).await?;

        Ok(llm_response)
    }

    let chain_result = trace_node(
        "agent_chain",
        RunType::Chain,
        "What is Rust?".to_string(),
        agent_chain,
    ).await?;

    println!("Chain Result: {}\n", chain_result);
    println!("Check LangSmith dashboard to see the hierarchical trace!");

    Ok(())
}

