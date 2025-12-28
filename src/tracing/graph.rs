use crate::error::Result;
use crate::models::run::RunType;
use crate::tracing::scope::RunScope;
use crate::tracing::tracer::Tracer;
use serde_json::Value;

/// Opinionated tracing helpers to build a Graph-style hierarchy in LangSmith:
/// - Root run named `Graph` (RunType::Chain)
/// - Step runs under root (e.g. `chatbot`, `should_continue`, `tools`)
/// - Nested runs under steps (e.g. `ChatOpenAI`, tool runs)
///
/// This is intentionally generic: inputs/outputs are `serde_json::Value` so any app can
/// provide LangSmith-compatible payloads such as `{ \"messages\": [...] }`.
pub struct GraphTrace {
    root: RunScope,
}

impl GraphTrace {
    /// Starts the root Graph run (name: `Graph`, type: Chain) and POSTs it.
    pub async fn start_root(inputs: Value, thread_id: Option<String>) -> Result<Self> {
        let mut root = RunScope::root_value("Graph", RunType::Chain, inputs);
        if let Some(tid) = thread_id {
            root = root.with_thread_id(tid);
        }
        root.post_start().await?;
        Ok(Self { root })
    }

    pub fn root_scope(&self) -> &RunScope {
        &self.root
    }

    pub fn root_tracer(&self) -> &Tracer {
        self.root.tracer()
    }

    /// Starts a top-level step/node under the root run (POSTs it) and returns the scope.
    /// Use this for nodes like "chatbot", "tools", etc. that may contain nested runs.
    pub async fn start_node_iteration(&self, node_name: &str, inputs: Value) -> Result<RunScope> {
        let mut step = self.root.child_value(node_name, RunType::Chain, inputs);
        step.post_start().await?;
        Ok(step)
    }

    /// Traces an LLM call within a parent node (e.g., within "chatbot").
    /// `llm_name` is typically "ChatOpenAI" or similar.
    /// `model_name` is optional (e.g., "gpt-4o-mini") and will be added to inputs if provided.
    pub async fn trace_llm_call(
        &self,
        parent_node: &RunScope,
        llm_name: &str,
        inputs: Value,
        outputs: Value,
        model_name: Option<&str>,
    ) -> Result<()> {
        let mut llm_inputs = inputs;
        if let Some(model) = model_name {
            if let Some(obj) = llm_inputs.as_object_mut() {
                obj.insert("model".to_string(), serde_json::json!(model));
            }
        }
        let mut llm = parent_node.child_value(llm_name, RunType::Llm, llm_inputs);
        llm.post_start().await?;
        llm.end_ok(outputs).await
    }

    /// Traces a routing/decision step (e.g., "should_continue").
    pub async fn trace_decision(
        &self,
        parent_node: &RunScope,
        decision_name: &str,
        inputs: Value,
        outputs: Value,
    ) -> Result<()> {
        let mut decision = parent_node.child_value(decision_name, RunType::Chain, inputs);
        decision.post_start().await?;
        decision.end_ok(outputs).await
    }

    /// Traces a tool call within a parent node (e.g., within "tools").
    /// `tool_name` should be the actual tool name (e.g., "calculator").
    pub async fn trace_tool_call(
        &self,
        parent_node: &RunScope,
        tool_name: &str,
        inputs: Value,
        outputs: Value,
    ) -> Result<()> {
        // Format tool name as "tool/{name}" to match LangGraph Python convention
        let formatted_name = format!("tool/{}", tool_name);
        let mut tool = parent_node.child_value(&formatted_name, RunType::Tool, inputs);
        tool.post_start().await?;
        tool.end_ok(outputs).await
    }

    /// Ends the root run with the provided outputs (PATCH). Consumes self.
    pub async fn end_root(self, outputs: Value) -> Result<()> {
        self.root.end_ok(outputs).await
    }
}


