use crate::observability::observer::Observer;
use crate::models::run::RunType;
use crate::tracing::decorator::trace_node;
use crate::error::Result;
use serde::Serialize;
use std::future::Future;
use std::sync::Arc;

/// Wrapper that makes a node function observable
pub struct ObservableNodeWrapper {
    name: String,
    run_type: RunType,
    observers: Vec<Arc<dyn Observer>>,
}

impl ObservableNodeWrapper {
    pub fn new(name: impl Into<String>, run_type: RunType) -> Self {
        Self {
            name: name.into(),
            run_type,
            observers: Vec::new(),
        }
    }

    pub fn with_observer(mut self, observer: Arc<dyn Observer>) -> Self {
        self.observers.push(observer);
        self
    }

    /// Execute a node function with tracing and observation
    pub async fn execute<F, Fut, I, O>(
        &self,
        inputs: I,
        f: F,
    ) -> Result<O>
    where
        F: FnOnce(I) -> Fut,
        Fut: Future<Output = Result<O>>,
        I: Serialize,
        O: Serialize,
    {
        use serde_json::to_value;

        // Notify observers of start
        let inputs_value = to_value(&inputs).unwrap_or_default();
        for observer in &self.observers {
            observer.on_node_start(&self.name, &inputs_value);
        }

        // Execute with tracing
        let result = trace_node(&self.name, self.run_type.clone(), inputs, f).await;

        // Notify observers of end or error
        match &result {
            Ok(output) => {
                let outputs_value = to_value(output).unwrap_or_default();
                for observer in &self.observers {
                    observer.on_node_end(&self.name, &outputs_value);
                }
            }
            Err(e) => {
                for observer in &self.observers {
                    observer.on_node_error(&self.name, &e.to_string());
                }
            }
        }

        result
    }
}

