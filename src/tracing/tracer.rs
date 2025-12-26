use crate::client::LangSmithClient;
use crate::config::Config;
use crate::error::Result;
use crate::models::run::{Run, RunType, RunUpdate};
use crate::tracing::context::TraceContext;
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

pub struct Tracer {
    pub(crate) run: Run,
    client: Option<Arc<LangSmithClient>>,
    #[allow(dead_code)]
    parent_tracer: Option<Arc<Tracer>>,
}

impl Tracer {
    pub fn new(name: impl Into<String>, run_type: RunType, inputs: Value) -> Self {
        let mut run = Run::new(name.into(), run_type, inputs);
        
        // Set session_name from config if available (project name, not UUID)
        if let Ok(config) = Config::get() {
            if let Some(project) = &config.project {
                run.session_name = Some(project.clone());
            }
        }

        Self {
            run,
            client: None,
            parent_tracer: None,
        }
    }

    pub fn with_client(mut self, client: Arc<LangSmithClient>) -> Self {
        self.client = Some(client);
        self
    }

    pub fn with_thread_id(mut self, thread_id: String) -> Self {
        self.run.thread_id = Some(thread_id);
        self
    }

    pub fn with_context(mut self, context: &TraceContext) -> Self {
        self.run.trace_id = Some(context.trace_id);
        if let Some(parent_id) = context.parent_run_id {
            self.run.parent_run_id = Some(parent_id);
        }
        if let Some(ref dotted_order) = context.dotted_order {
            self.run.dotted_order = Some(dotted_order.clone());
        }
        if let Some(ref thread_id) = context.thread_id {
            self.run.thread_id = Some(thread_id.clone());
        }
        if let Some(ref session_name) = context.session_name {
            self.run.session_name = Some(session_name.clone());
        }
        self
    }

    pub fn create_child(
        &self,
        name: impl Into<String>,
        run_type: RunType,
        inputs: Value,
    ) -> Self {
        let mut child = Self::new(name, run_type, inputs);
        
        // Set parent relationship
        child.run.parent_run_id = Some(self.run.id);
        child.run.trace_id = self.run.trace_id.or(Some(self.run.id));
        
        // Generate dotted_order
        let parent_dotted_order = self.run.dotted_order.as_deref();
        child.run.dotted_order = Some(child.run.generate_dotted_order(parent_dotted_order));
        
        // Inherit thread_id
        child.run.thread_id = self.run.thread_id.clone();
        
        // Inherit session_name
        child.run.session_name = self.run.session_name.clone();
        
        // Share client if available
        if let Some(client) = &self.client {
            child.client = Some(Arc::clone(client));
        }

        child
    }

    pub async fn post(&mut self) -> Result<()> {
        // Initialize trace_id if this is the root run
        if self.run.trace_id.is_none() {
            self.run.trace_id = Some(self.run.id);
            self.run.dotted_order = Some(self.run.generate_dotted_order(None));
        }

        // Get or create client
        let client = if let Some(client) = &self.client {
            Arc::clone(client)
        } else {
            Arc::new(LangSmithClient::new()?)
        };

        // Post run - await to ensure it completes
        if let Err(e) = client.post_run(&self.run).await {
            eprintln!("LangSmith tracing error: {}", e);
        }

        Ok(())
    }

    pub async fn patch(&self) -> Result<()> {
        let client = if let Some(client) = &self.client {
            Arc::clone(client)
        } else {
            Arc::new(LangSmithClient::new()?)
        };

        let run_id = self.run.id;
        let updates = RunUpdate::from(&self.run);
        
        // Patch run - await to ensure it completes
        if let Err(e) = client.patch_run(run_id, &updates).await {
            eprintln!("LangSmith tracing error: {}", e);
        }

        Ok(())
    }

    pub fn end(&mut self, outputs: Value) {
        self.run.end(outputs);
    }

    pub fn set_error(&mut self, error: &str) {
        self.run.set_error(error);
    }

    pub fn run_id(&self) -> Uuid {
        self.run.id
    }

    pub fn trace_id(&self) -> Option<Uuid> {
        self.run.trace_id
    }

    pub fn name(&self) -> &str {
        &self.run.name
    }

    pub fn run_type(&self) -> &RunType {
        &self.run.run_type
    }

    pub fn parent_run_id(&self) -> Option<Uuid> {
        self.run.parent_run_id
    }

    pub fn dotted_order(&self) -> Option<&String> {
        self.run.dotted_order.as_ref()
    }

    pub fn thread_id(&self) -> Option<&String> {
        self.run.thread_id.as_ref()
    }

    pub fn session_name(&self) -> Option<&String> {
        self.run.session_name.as_ref()
    }

    pub fn context(&self) -> TraceContext {
        TraceContext {
            trace_id: self.run.trace_id.unwrap_or(self.run.id),
            parent_run_id: self.run.parent_run_id,
            dotted_order: self.run.dotted_order.clone(),
            thread_id: self.run.thread_id.clone(),
            session_name: self.run.session_name.clone(),
        }
    }
}

impl Clone for Tracer {
    fn clone(&self) -> Self {
        Self {
            run: self.run.clone(),
            client: self.client.as_ref().map(Arc::clone),
            parent_tracer: None, // Don't clone parent to avoid cycles
        }
    }
}

