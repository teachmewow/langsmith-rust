use langsmith_rust::factories::TracerFactory;
use langsmith_rust::models::run::RunType;
use langsmith_rust::tracing::context::TraceContext;
use serde_json::json;
use uuid::Uuid;

#[test]
fn test_tracer_factory_create() {
    let tracer = TracerFactory::create("Test".to_string(), RunType::Chain, json!({}));
    
    assert_eq!(tracer.name(), "Test");
    assert_eq!(tracer.run_type(), &RunType::Chain);
}

#[test]
fn test_tracer_factory_create_with_thread() {
    let tracer = TracerFactory::create_with_thread(
        "Test".to_string(),
        RunType::Chain,
        json!({}),
        "thread-123".to_string(),
    );
    
    assert_eq!(tracer.thread_id(), Some(&"thread-123".to_string()));
}

#[test]
fn test_tracer_factory_create_root() {
    let tracer = TracerFactory::create_root("Root".to_string(), RunType::Chain, json!({}));
    
    assert!(tracer.trace_id().is_some());
    assert!(tracer.dotted_order().is_some());
    // Verify dotted_order format
    let dotted = tracer.dotted_order().unwrap();
    assert!(dotted.contains('Z'));
    assert!(dotted.len() > 20);
}

#[test]
fn test_tracer_factory_create_with_context() {
    let trace_id = Uuid::new_v4();
    let context = TraceContext::new(trace_id)
        .with_thread_id("thread-123".to_string());
    
    let tracer = TracerFactory::create_with_context(
        "Test".to_string(),
        RunType::Chain,
        json!({}),
        &context,
    );
    
    assert_eq!(tracer.trace_id(), Some(trace_id));
    assert_eq!(tracer.thread_id(), Some(&"thread-123".to_string()));
}

#[test]
fn test_tracer_factory_create_for_node() {
    let tracer = TracerFactory::create_for_node(
        "Node".to_string(),
        RunType::Llm,
        json!({}),
        None,
    );
    
    assert!(tracer.trace_id().is_some());
}

#[test]
fn test_tracer_factory_create_for_node_with_parent() {
    let trace_id = Uuid::new_v4();
    let parent_context = TraceContext::new(trace_id);
    
    let tracer = TracerFactory::create_for_node(
        "Child Node".to_string(),
        RunType::Tool,
        json!({}),
        Some(&parent_context),
    );
    
    assert_eq!(tracer.trace_id(), Some(trace_id));
}

