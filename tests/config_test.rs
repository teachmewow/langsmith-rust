use langsmith_rust::config::Config;
use std::env;

#[test]
fn test_config_from_env() {
    Config::reset();
    // Ensure clean state
    env::remove_var("LANGSMITH_TRACING");
    env::remove_var("LANGSMITH_ENDPOINT");
    env::remove_var("LANGSMITH_API_KEY");
    env::remove_var("LANGSMITH_PROJECT");
    
    env::set_var("LANGSMITH_TRACING", "true");
    env::set_var("LANGSMITH_ENDPOINT", "https://test.api.smith.langchain.com");
    env::set_var("LANGSMITH_API_KEY", "test-api-key");
    env::set_var("LANGSMITH_PROJECT", "test-project");

    let config = Config::from_env_no_dotenv();
    assert!(config.is_ok());
    let config = config.unwrap();
    
    assert_eq!(config.tracing_enabled, true);
    assert_eq!(config.endpoint, "https://test.api.smith.langchain.com");
    assert_eq!(config.api_key, "test-api-key");
    assert_eq!(config.project, Some("test-project".to_string()));
}

#[test]
fn test_config_defaults() {
    Config::reset();
    // Clean all vars first
    env::remove_var("LANGSMITH_TRACING");
    env::remove_var("LANGSMITH_ENDPOINT");
    env::remove_var("LANGSMITH_API_KEY");
    env::remove_var("LANGSMITH_PROJECT");
    
    // Set only required var
    env::set_var("LANGSMITH_API_KEY", "test-key");

    let config = Config::from_env_no_dotenv();
    assert!(config.is_ok());
    let config = config.unwrap();
    
    assert_eq!(config.tracing_enabled, false);
    assert_eq!(config.endpoint, "https://api.smith.langchain.com");
}

#[test]
fn test_config_missing_api_key() {
    Config::reset();
    // Ensure API key is not set
    env::remove_var("LANGSMITH_API_KEY");
    // But set other vars to avoid .env file interference
    env::set_var("LANGSMITH_TRACING", "false");
    
    let config = Config::from_env_no_dotenv();
    assert!(config.is_err());
    assert!(config.unwrap_err().to_string().contains("LANGSMITH_API_KEY"));
}

#[test]
fn test_is_tracing_enabled() {
    Config::reset();
    env::remove_var("LANGSMITH_TRACING");
    env::set_var("LANGSMITH_TRACING", "true");
    env::set_var("LANGSMITH_API_KEY", "test-key");
    
    // Use from_env_no_dotenv to avoid .env file interference
    let config = Config::from_env_no_dotenv().unwrap();
    assert!(config.tracing_enabled);
    
    Config::reset();
    env::remove_var("LANGSMITH_TRACING");
    env::set_var("LANGSMITH_TRACING", "false");
    let config = Config::from_env_no_dotenv().unwrap();
    assert!(!config.tracing_enabled);
}

