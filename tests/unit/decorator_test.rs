use langsmith_rust::models::run::RunType;
use langsmith_rust::tracing::decorator::trace_node;
use langsmith_rust::error::Result;
use serde_json::json;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    async fn test_function(input: i32) -> Result<i32> {
        Ok(input * 2)
    }

    #[tokio::test]
    async fn test_trace_node_success() {
        // Disable tracing for unit tests
        env::set_var("LANGSMITH_TRACING", "false");
        env::set_var("LANGSMITH_API_KEY", "test-key");
        
        let result = trace_node(
            "test_node",
            RunType::Runnable,
            5,
            test_function,
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10);
    }

    async fn test_function_error(_input: i32) -> Result<i32> {
        Err(langsmith_rust::error::LangSmithError::Other("Test error".to_string()))
    }

    #[tokio::test]
    async fn test_trace_node_error() {
        env::set_var("LANGSMITH_TRACING", "false");
        env::set_var("LANGSMITH_API_KEY", "test-key");
        
        let result = trace_node(
            "test_node",
            RunType::Runnable,
            5,
            test_function_error,
        ).await;
        
        assert!(result.is_err());
    }

    #[test]
    fn test_trace_node_sync() {
        use langsmith_rust::tracing::decorator::trace_node_sync;
        
        env::set_var("LANGSMITH_TRACING", "false");
        env::set_var("LANGSMITH_API_KEY", "test-key");
        
        fn sync_function(input: i32) -> Result<i32> {
            Ok(input * 3)
        }
        
        let result = trace_node_sync(
            "sync_node",
            RunType::Runnable,
            4,
            sync_function,
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 12);
    }
}

