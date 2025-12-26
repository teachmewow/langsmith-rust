use langsmith_rust::models::run::{Run, RunType};
use serde_json::json;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_creation() {
        let run = Run::new("Test Run", RunType::Chain, json!({"input": "test"}));
        
        assert_eq!(run.name, "Test Run");
        assert_eq!(run.run_type, RunType::Chain);
        assert_eq!(run.inputs, json!({"input": "test"}));
        assert!(run.outputs.is_none());
        assert!(run.trace_id.is_none());
    }

    #[test]
    fn test_run_type_as_str() {
        assert_eq!(RunType::Chain.as_str(), "chain");
        assert_eq!(RunType::Llm.as_str(), "llm");
        assert_eq!(RunType::Tool.as_str(), "tool");
        assert_eq!(RunType::Custom("custom_type".to_string()).as_str(), "custom_type");
    }

    #[test]
    fn test_run_generate_dotted_order() {
        let run = Run::new("Test", RunType::Chain, json!({}));
        let dotted_order = run.generate_dotted_order(None);
        
        // Should contain timestamp and UUID
        assert!(dotted_order.contains('Z'));
        assert!(dotted_order.len() > 20);
    }

    #[test]
    fn test_run_generate_dotted_order_with_parent() {
        let parent = Run::new("Parent", RunType::Chain, json!({}));
        let parent_dotted = parent.generate_dotted_order(None);
        
        let child = Run::new("Child", RunType::Llm, json!({}));
        let child_dotted = child.generate_dotted_order(Some(&parent_dotted));
        
        // Should contain parent dotted order
        assert!(child_dotted.starts_with(&parent_dotted));
        assert!(child_dotted.contains('.'));
    }

    #[test]
    fn test_run_set_error() {
        let mut run = Run::new("Test", RunType::Chain, json!({}));
        run.set_error("Test error");
        
        assert_eq!(run.error, Some("Test error".to_string()));
    }

    #[test]
    fn test_run_end() {
        let mut run = Run::new("Test", RunType::Chain, json!({}));
        run.end(json!({"result": "success"}));
        
        assert!(run.outputs.is_some());
        assert!(run.end_time.is_some());
    }

    #[test]
    fn test_run_update_from_run() {
        let mut run = Run::new("Test", RunType::Chain, json!({}));
        run.end(json!({"result": "success"}));
        run.set_error("Error");
        
        let update = langsmith_rust::models::run::RunUpdate::from(&run);
        
        assert!(update.outputs.is_some());
        assert!(update.end_time.is_some());
        assert_eq!(update.error, Some("Error".to_string()));
    }
}

