use crate::config::Config;
use crate::error::{LangSmithError, Result};
use crate::models::run::{Run, RunUpdate};
use reqwest::Client;
use uuid::Uuid;

pub struct LangSmithClient {
    client: Client,
    config: Config,
}

impl LangSmithClient {
    pub fn new() -> Result<Self> {
        let config = Config::get()?;
        let client = Client::new();
        Ok(Self { client, config })
    }

    pub fn with_config(config: Config) -> Self {
        let client = Client::new();
        Self { client, config }
    }

    pub async fn post_run(&self, run: &Run) -> Result<()> {
        if !self.config.tracing_enabled {
            return Err(LangSmithError::TracingDisabled);
        }

        let url = format!("{}/runs", self.config.endpoint);
        
        let mut request = self
            .client
            .post(&url)
            .header("x-api-key", &self.config.api_key)
            .json(run);

        if let Some(tenant_id) = &self.config.tenant_id {
            request = request.header("x-tenant-id", tenant_id);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(LangSmithError::Other(format!(
                "HTTP {}: {}",
                status.as_u16(),
                text
            )));
        }

        Ok(())
    }

    pub async fn patch_run(&self, run_id: Uuid, updates: &RunUpdate) -> Result<()> {
        if !self.config.tracing_enabled {
            return Err(LangSmithError::TracingDisabled);
        }

        let url = format!("{}/runs/{}", self.config.endpoint, run_id);
        
        let mut request = self
            .client
            .patch(&url)
            .header("x-api-key", &self.config.api_key)
            .json(updates);

        if let Some(tenant_id) = &self.config.tenant_id {
            request = request.header("x-tenant-id", tenant_id);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(LangSmithError::Other(format!(
                "HTTP {}: {}",
                status.as_u16(),
                text
            )));
        }

        Ok(())
    }
}

