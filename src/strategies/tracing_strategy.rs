use crate::error::Result;
use crate::models::run::Run;
use async_trait::async_trait;

/// Strategy pattern for different tracing implementations
#[async_trait]
pub trait TracingStrategy: Send + Sync {
    /// Trace the start of a run
    async fn trace_start(&self, run: &Run) -> Result<()>;
    
    /// Trace the end of a run
    async fn trace_end(&self, run: &Run) -> Result<()>;
    
    /// Trace an error in a run
    async fn trace_error(&self, run: &Run, error: &str) -> Result<()>;
}

/// Async tracing strategy (default)
pub struct AsyncTracingStrategy {
    // Can hold client or other dependencies
}

impl AsyncTracingStrategy {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl TracingStrategy for AsyncTracingStrategy {
    async fn trace_start(&self, run: &Run) -> Result<()> {
        use crate::client::LangSmithClient;
        let client = LangSmithClient::new()?;
        client.post_run(run).await
    }

    async fn trace_end(&self, run: &Run) -> Result<()> {
        use crate::client::LangSmithClient;
        use crate::models::run::RunUpdate;
        let client = LangSmithClient::new()?;
        let updates = RunUpdate::from(run);
        client.patch_run(run.id, &updates).await
    }

    async fn trace_error(&self, run: &Run, error: &str) -> Result<()> {
        use crate::client::LangSmithClient;
        use crate::models::run::RunUpdate;
        let client = LangSmithClient::new()?;
        let mut updates = RunUpdate::from(run);
        updates.error = Some(error.to_string());
        client.patch_run(run.id, &updates).await
    }
}

/// Sync tracing strategy (uses blocking runtime)
pub struct SyncTracingStrategy {
    // Can hold client or other dependencies
}

impl SyncTracingStrategy {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl TracingStrategy for SyncTracingStrategy {
    async fn trace_start(&self, run: &Run) -> Result<()> {
        use crate::client::LangSmithClient;
        let rt = tokio::runtime::Runtime::new().unwrap();
        let client = LangSmithClient::new()?;
        rt.block_on(client.post_run(run))
    }

    async fn trace_end(&self, run: &Run) -> Result<()> {
        use crate::client::LangSmithClient;
        use crate::models::run::RunUpdate;
        let rt = tokio::runtime::Runtime::new().unwrap();
        let client = LangSmithClient::new()?;
        let updates = RunUpdate::from(run);
        rt.block_on(client.patch_run(run.id, &updates))
    }

    async fn trace_error(&self, run: &Run, error: &str) -> Result<()> {
        use crate::client::LangSmithClient;
        use crate::models::run::RunUpdate;
        let rt = tokio::runtime::Runtime::new().unwrap();
        let client = LangSmithClient::new()?;
        let mut updates = RunUpdate::from(run);
        updates.error = Some(error.to_string());
        rt.block_on(client.patch_run(run.id, &updates))
    }
}

