use std::env;

pub fn setup_test_env() {
    env::set_var("LANGSMITH_TRACING", "false");
    env::set_var("LANGSMITH_API_KEY", "test-api-key");
    env::set_var("LANGSMITH_ENDPOINT", "https://test.api.smith.langchain.com");
    env::set_var("LANGSMITH_PROJECT", "test-project");
}

pub fn cleanup_test_env() {
    env::remove_var("LANGSMITH_TRACING");
    env::remove_var("LANGSMITH_API_KEY");
    env::remove_var("LANGSMITH_ENDPOINT");
    env::remove_var("LANGSMITH_PROJECT");
}

