// Example: Test LLM tracing with OpenAI
use langsmith_rust::{trace_node, RunType, Tracer, Result};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize (loads .env automatically)
    langsmith_rust::init();

    println!("Testing LangSmith tracing with OpenAI LLM call\n");

    // Get OpenAI API key from environment
    let openai_api_key = std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set in .env");

    // Example 1: Manual tracing with real OpenAI call
    println!("Example 1: Manual tracing");
    println!("Calling OpenAI with: 'Oi, tudo bem?'\n");

    let mut tracer = Tracer::new(
        "LLM Chat Pipeline",
        RunType::Chain,
        json!({
            "user_message": "Oi, tudo bem?",
            "model": "gpt-4o-mini"
        }),
    );

    tracer.post().await?;

    // Create child run for LLM call
    let mut llm_run = tracer.create_child(
        "OpenAI Chat Completion",
        RunType::Llm,
        json!({
            "messages": [
                {"role": "user", "content": "Oi, tudo bem?"}
            ],
            "model": "gpt-4o-mini"
        }),
    );
    llm_run.post().await?;

    // Make actual OpenAI API call
    let start_time = std::time::Instant::now();
    let response = call_openai(&openai_api_key, "Oi, tudo bem?").await?;
    let duration = start_time.elapsed();

    println!("Response from OpenAI: {}", response);
    println!("Duration: {:?}\n", duration);

    // Update run with outputs
    llm_run.end(json!({
        "completion": response,
        "duration_ms": duration.as_millis(),
        "model": "gpt-4o-mini"
    }));
    llm_run.patch().await?;

    // End parent run
    tracer.end(json!({
        "answer": response,
        "duration_ms": duration.as_millis()
    }));
    tracer.patch().await?;

    println!("Manual tracing completed! Check LangSmith dashboard.\n");

    // Example 2: Using trace_node helper
    println!("Example 2: Using trace_node helper");
    println!("Calling OpenAI with: 'Como você está?'\n");

    async fn llm_node(message: String) -> Result<String> {
        let openai_api_key = std::env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY must be set");
        call_openai(&openai_api_key, &message).await
    }

    let result = trace_node(
        "OpenAI Chat Completion (Helper)",
        RunType::Llm,
        "Como você está?".to_string(),
        llm_node,
    )
    .await?;

    println!("Response from OpenAI: {}", result);
    println!("Automatic tracing completed! Check LangSmith dashboard.\n");

    Ok(())
}

async fn call_openai(api_key: &str, message: &str) -> Result<String> {
    let client = reqwest::Client::new();
    
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "gpt-4o-mini",
            "messages": [
                {"role": "user", "content": message}
            ],
            "temperature": 0.7,
            "max_tokens": 150
        }))
        .send()
        .await
        .map_err(|e| langsmith_rust::LangSmithError::Other(format!("OpenAI API error: {}", e)))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(langsmith_rust::LangSmithError::Other(format!(
            "OpenAI API error {}: {}",
            status.as_u16(),
            text
        )));
    }

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| langsmith_rust::LangSmithError::Other(format!("JSON parse error: {}", e)))?;

    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| langsmith_rust::LangSmithError::Other("Invalid response format".to_string()))?
        .to_string();

    Ok(content)
}

