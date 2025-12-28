use crate::error::{LangSmithError, Result};
use crate::models::run::RunType;
use crate::tracing::tracer::Tracer;
use crate::tracing::TraceContext;
use crate::utils::serialization::{ensure_inputs_object, ensure_outputs_object};
use serde::Serialize;
use serde_json::Value;

/// Ergonomic wrapper around `Tracer` that standardizes:
/// - inputs/outputs serialization
/// - post/end/patch flow
/// - error handling (best-effort patching)
///
/// This is intended to keep application code small and declarative.
pub struct RunScope {
    tracer: Tracer,
    posted: bool,
}

impl RunScope {
    pub fn root<I: Serialize>(name: &str, run_type: RunType, inputs: I) -> Result<Self> {
        let inputs_value =
            ensure_inputs_object(inputs).map_err(LangSmithError::Serialization)?;
        Ok(Self {
            tracer: Tracer::new(name, run_type, inputs_value),
            posted: false,
        })
    }

    pub fn root_value(name: &str, run_type: RunType, inputs: Value) -> Self {
        Self {
            tracer: Tracer::new(name, run_type, inputs),
            posted: false,
        }
    }

    pub fn with_thread_id(mut self, thread_id: impl Into<String>) -> Self {
        self.tracer = self.tracer.with_thread_id(thread_id.into());
        self
    }

    pub fn with_context(mut self, ctx: &TraceContext) -> Self {
        self.tracer = self.tracer.with_context(ctx);
        self
    }

    pub fn tracer(&self) -> &Tracer {
        &self.tracer
    }

    pub fn tracer_mut(&mut self) -> &mut Tracer {
        &mut self.tracer
    }

    pub fn child<I: Serialize>(&self, name: &str, run_type: RunType, inputs: I) -> Result<Self> {
        let inputs_value =
            ensure_inputs_object(inputs).map_err(LangSmithError::Serialization)?;
        Ok(Self {
            tracer: self.tracer.create_child(name, run_type, inputs_value),
            posted: false,
        })
    }

    pub fn child_value(&self, name: &str, run_type: RunType, inputs: Value) -> Self {
        Self {
            tracer: self.tracer.create_child(name, run_type, inputs),
            posted: false,
        }
    }

    /// Posts the run start to LangSmith. Safe to call multiple times.
    pub async fn post_start(&mut self) -> Result<()> {
        if self.posted {
            return Ok(());
        }
        self.tracer.post().await?;
        self.posted = true;
        Ok(())
    }

    /// Ends the run successfully and PATCHes it (best-effort).
    pub async fn end_ok<O: Serialize>(mut self, outputs: O) -> Result<()> {
        let outputs_value =
            ensure_outputs_object(outputs).map_err(LangSmithError::Serialization)?;
        self.tracer.end(outputs_value);
        let _ = self.tracer.patch().await;
        Ok(())
    }

    /// Ends the run with error and PATCHes it (best-effort).
    pub async fn end_error(mut self, error: impl ToString, outputs: Option<Value>) -> Result<()> {
        self.tracer.set_error(&error.to_string());
        self.tracer.end(outputs.unwrap_or_else(|| serde_json::json!({})));
        let _ = self.tracer.patch().await;
        Ok(())
    }
}


