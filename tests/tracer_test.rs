use langsmith_rust::models::run::RunType;
use langsmith_rust::tracing::tracer::Tracer;
use langsmith_rust::tracing::context::TraceContext;
use serde_json::json;
use uuid::Uuid;

#[test]
fn test_tracer_creation() {
    let tracer = Tracer::new("Test Tracer".to_string(), RunType::Chain, json!({"input": "test"}));
    
    assert_eq!(tracer.name(), "Test Tracer");
    assert_eq!(tracer.run_type(), &RunType::Chain);
}

#[test]
fn test_tracer_with_thread_id() {
    let tracer = Tracer::new("Test".to_string(), RunType::Chain, json!({}))
        .with_thread_id("thread-123".to_string());
    
    assert_eq!(tracer.thread_id(), Some(&"thread-123".to_string()));
}

#[test]
fn test_tracer_create_child() {
    let parent = Tracer::new("Parent".to_string(), RunType::Chain, json!({}));
    let child = parent.create_child("Child".to_string(), RunType::Llm, json!({}));
    
    assert_eq!(child.parent_run_id(), Some(parent.run_id()));
    assert_eq!(child.trace_id(), Some(parent.run_id()));
    assert!(child.dotted_order().is_some());
}

#[test]
fn test_tracer_with_context() {
    let trace_id = Uuid::new_v4();
    let context = TraceContext::new(trace_id)
        .with_thread_id("thread-123".to_string())
        .with_session_name("test-project".to_string());
    
    let tracer = Tracer::new("Test".to_string(), RunType::Chain, json!({}))
        .with_context(&context);
    
    assert_eq!(tracer.trace_id(), Some(trace_id));
    assert_eq!(tracer.thread_id(), Some(&"thread-123".to_string()));
    assert_eq!(tracer.session_name(), Some(&"test-project".to_string()));
}

#[test]
fn test_tracer_context() {
    let tracer = Tracer::new("Test".to_string(), RunType::Chain, json!({}));
    let context = tracer.context();
    
    assert!(context.trace_id == tracer.run_id() || tracer.trace_id() == Some(context.trace_id));
}

#[test]
fn test_tracer_run_id() {
    let tracer = Tracer::new("Test".to_string(), RunType::Chain, json!({}));
    let run_id = tracer.run_id();
    
    assert_eq!(run_id, tracer.run_id());
}

