#[cfg(test)]
mod tests {
    use langsmith_rust::config::Config;
    use std::env;

    #[test]
    fn test_config_from_env() {
        // Set test environment variables
        env::set_var("LANGSMITH_TRACING", "true");
        env::set_var("LANGSMITH_ENDPOINT", "https://test.api.smith.langchain.com");
        env::set_var("LANGSMITH_API_KEY", "test-api-key");
        env::set_var("LANGSMITH_PROJECT", "test-project");

        let config = Config::from_env();
        assert!(config.is_ok());
        let config = config.unwrap();
        
        assert_eq!(config.tracing_enabled, true);
        assert_eq!(config.endpoint, "https://test.api.smith.langchain.com");
        assert_eq!(config.api_key, "test-api-key");
        assert_eq!(config.project, Some("test-project".to_string()));
    }

    #[test]
    fn test_config_defaults() {
        // Clear environment variables
        env::remove_var("LANGSMITH_TRACING");
        env::remove_var("LANGSMITH_ENDPOINT");
        env::set_var("LANGSMITH_API_KEY", "test-key");

        let config = Config::from_env();
        assert!(config.is_ok());
        let config = config.unwrap();
        
        assert_eq!(config.tracing_enabled, false);
        assert_eq!(config.endpoint, "https://api.smith.langchain.com");
    }

    #[test]
    fn test_config_missing_api_key() {
        env::remove_var("LANGSMITH_API_KEY");
        
        let config = Config::from_env();
        assert!(config.is_err());
    }

    #[test]
    fn test_is_tracing_enabled() {
        env::set_var("LANGSMITH_TRACING", "true");
        env::set_var("LANGSMITH_API_KEY", "test-key");
        
        assert!(Config::is_tracing_enabled());
        
        env::set_var("LANGSMITH_TRACING", "false");
        assert!(!Config::is_tracing_enabled());
    }
}

