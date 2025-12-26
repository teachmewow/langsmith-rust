use crate::error::{LangSmithError, Result};
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Config {
    pub tracing_enabled: bool,
    pub endpoint: String,
    pub api_key: String,
    pub project: Option<String>,
    pub tenant_id: Option<String>,
}

static CONFIG: Lazy<Mutex<Option<Config>>> = Lazy::new(|| Mutex::new(None));

impl Config {
    pub fn from_env() -> Result<Self> {
        // Try to load .env file (ignore errors if it doesn't exist)
        let _ = dotenvy::dotenv();

        let tracing_enabled = std::env::var("LANGSMITH_TRACING")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        let endpoint = std::env::var("LANGSMITH_ENDPOINT")
            .unwrap_or_else(|_| "https://api.smith.langchain.com".to_string());

        let api_key = std::env::var("LANGSMITH_API_KEY")
            .map_err(|_| LangSmithError::Config("LANGSMITH_API_KEY not set".to_string()))?;

        let project = std::env::var("LANGSMITH_PROJECT").ok();
        let tenant_id = std::env::var("LANGSMITH_TENANT_ID").ok();

        Ok(Config {
            tracing_enabled,
            endpoint,
            api_key,
            project,
            tenant_id,
        })
    }

    pub fn get() -> Result<Self> {
        let mut config = CONFIG.lock().unwrap();
        if config.is_none() {
            *config = Some(Self::from_env()?);
        }
        Ok(config.as_ref().unwrap().clone())
    }

    pub fn is_tracing_enabled() -> bool {
        Self::get()
            .map(|c| c.tracing_enabled)
            .unwrap_or(false)
    }
}

