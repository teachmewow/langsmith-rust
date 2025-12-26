use langsmith_rust::models::run::RunType;
use langsmith_rust::factories::TracerFactory;
use serde_json::json;
use std::env;

#[tokio::test]
async fn test_tracer_hierarchy() {
    env::set_var("LANGSMITH_TRACING", "false");
    env::set_var("LANGSMITH_API_KEY", "test-key");
    
    let parent = TracerFactory::create_root("Parent".to_string(), RunType::Chain, json!({"input": "test"}));
    let child = parent.create_child("Child".to_string(), RunType::Llm, json!({"messages": []}));
    let grandchild = child.create_child("Grandchild".to_string(), RunType::Tool, json!({"tool": "test"}));
    
    assert_eq!(child.parent_run_id(), Some(parent.run_id()));
    assert_eq!(grandchild.parent_run_id(), Some(child.run_id()));
    assert_eq!(child.trace_id(), Some(parent.run_id()));
    assert_eq!(grandchild.trace_id(), Some(parent.run_id()));
    
    assert!(parent.dotted_order().is_some());
    assert!(child.dotted_order().is_some());
    assert!(grandchild.dotted_order().is_some());
    
    let parent_dotted = parent.dotted_order().unwrap();
    let child_dotted = child.dotted_order().unwrap();
    let grandchild_dotted = grandchild.dotted_order().unwrap();
    
    assert!(child_dotted.starts_with(parent_dotted));
    assert!(grandchild_dotted.starts_with(child_dotted));
}

#[tokio::test]
async fn test_trace_context_propagation() {
    env::set_var("LANGSMITH_TRACING", "false");
    env::set_var("LANGSMITH_API_KEY", "test-key");
    
    let parent = TracerFactory::create_root("Parent".to_string(), RunType::Chain, json!({}));
    let context = parent.context();
    
    let child = TracerFactory::create_for_node(
        "Child".to_string(),
        RunType::Llm,
        json!({}),
        Some(&context),
    );
    
    assert_eq!(child.trace_id(), Some(context.trace_id));
    assert_eq!(child.parent_run_id(), Some(parent.run_id()));
}

