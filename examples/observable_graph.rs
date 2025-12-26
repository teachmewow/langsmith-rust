// Example: Observable graph nodes using Observer pattern
use langsmith_rust::{
    observability::{ObservableNodeWrapper, Observer, LangSmithObserver},
    models::run::RunType,
    Result,
};
use serde_json::json;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    langsmith_rust::init();

    println!("Example: Observable graph nodes\n");

    // Create observer
    let observer: Arc<dyn Observer> = Arc::new(LangSmithObserver::new());

    // Create observable node wrapper
    let llm_node = ObservableNodeWrapper::new("llm_node", RunType::Llm)
        .with_observer(Arc::clone(&observer));

    // Execute node with observation
    async fn llm_function(input: String) -> Result<String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        Ok(format!("Processed: {}", input))
    }

    let result = llm_node.execute(
        "Hello, world!".to_string(),
        llm_function,
    ).await?;

    println!("Result: {}\n", result);

    // Example with multiple observers
    struct ConsoleObserver;

    impl Observer for ConsoleObserver {
        fn on_node_start(&self, node_name: &str, _inputs: &serde_json::Value) {
            println!("[ConsoleObserver] Node '{}' started", node_name);
        }

        fn on_node_end(&self, node_name: &str, _outputs: &serde_json::Value) {
            println!("[ConsoleObserver] Node '{}' completed", node_name);
        }

        fn on_node_error(&self, node_name: &str, error: &str) {
            println!("[ConsoleObserver] Node '{}' error: {}", node_name, error);
        }
    }

    let console_observer: Arc<dyn Observer> = Arc::new(ConsoleObserver);
    let multi_observer_node = ObservableNodeWrapper::new("multi_observer_node", RunType::Chain)
        .with_observer(Arc::clone(&observer))
        .with_observer(console_observer);

    let result2 = multi_observer_node.execute(
        json!({"data": "test"}),
        |input: serde_json::Value| async move {
            Ok(json!({"result": input["data"]}))
        },
    ).await?;

    println!("Multi-observer result: {}\n", result2);
    println!("Both observers were notified of node execution!");

    Ok(())
}

