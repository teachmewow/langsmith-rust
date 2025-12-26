use langsmith_rust::models::run::RunType;
use langsmith_rust::tracing::tracer::Tracer;
use langsmith_rust::factories::TracerFactory;
use serde_json::json;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tracer_hierarchy() {
        // Disable actual API calls for integration tests
        env::set_var("LANGSMITH_TRACING", "false");
        env::set_var("LANGSMITH_API_KEY", "test-key");
        
        let mut parent = TracerFactory::create_root("Parent", RunType::Chain, json!({"input": "test"}));
        let mut child = parent.create_child("Child", RunType::Llm, json!({"messages": []}));
        let grandchild = child.create_child("Grandchild", RunType::Tool, json!({"tool": "test"}));
        
        // Verify hierarchy
        assert_eq!(child.run.parent_run_id, Some(parent.run.id));
        assert_eq!(grandchild.run.parent_run_id, Some(child.run.id));
        assert_eq!(child.run.trace_id, Some(parent.run.id));
        assert_eq!(grandchild.run.trace_id, Some(parent.run.id));
        
        // Verify dotted_order hierarchy
        assert!(parent.run.dotted_order.is_some());
        assert!(child.run.dotted_order.is_some());
        assert!(grandchild.run.dotted_order.is_some());
        
        let parent_dotted = parent.run.dotted_order.as_ref().unwrap();
        let child_dotted = child.run.dotted_order.as_ref().unwrap();
        let grandchild_dotted = grandchild.run.dotted_order.as_ref().unwrap();
        
        assert!(child_dotted.starts_with(parent_dotted));
        assert!(grandchild_dotted.starts_with(child_dotted));
    }

    #[tokio::test]
    async fn test_trace_context_propagation() {
        env::set_var("LANGSMITH_TRACING", "false");
        env::set_var("LANGSMITH_API_KEY", "test-key");
        
        let parent = TracerFactory::create_root("Parent", RunType::Chain, json!({}));
        let context = parent.context();
        
        let child = TracerFactory::create_for_node(
            "Child",
            RunType::Llm,
            json!({}),
            Some(&context),
        );
        
        assert_eq!(child.run.trace_id, context.trace_id);
        assert_eq!(child.run.parent_run_id, Some(parent.run.id));
    }
}

