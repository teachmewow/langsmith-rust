use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunType {
    Chain,
    Llm,
    Tool,
    Retriever,
    Embedding,
    Prompt,
    Runnable,
    Custom(String),
}

impl RunType {
    pub fn as_str(&self) -> &str {
        match self {
            RunType::Chain => "chain",
            RunType::Llm => "llm",
            RunType::Tool => "tool",
            RunType::Retriever => "retriever",
            RunType::Embedding => "embedding",
            RunType::Prompt => "prompt",
            RunType::Runnable => "runnable",
            RunType::Custom(s) => s,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "run_type")]
    pub run_type: RunType,
    pub inputs: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Value>,
    #[serde(rename = "start_time")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(rename = "parent_run_id", skip_serializing_if = "Option::is_none")]
    pub parent_run_id: Option<Uuid>,
    #[serde(rename = "trace_id", skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<Uuid>,
    #[serde(rename = "dotted_order", skip_serializing_if = "Option::is_none")]
    pub dotted_order: Option<String>,
    #[serde(rename = "session_id", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "session_name", skip_serializing_if = "Option::is_none")]
    pub session_name: Option<String>,
    #[serde(rename = "thread_id", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, Value>,
    // Metrics
    #[serde(rename = "prompt_tokens", skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<u64>,
    #[serde(rename = "completion_tokens", skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<u64>,
    #[serde(rename = "total_tokens", skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<u64>,
    #[serde(rename = "total_cost", skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<f64>,
    #[serde(rename = "prompt_cost", skip_serializing_if = "Option::is_none")]
    pub prompt_cost: Option<f64>,
    #[serde(rename = "completion_cost", skip_serializing_if = "Option::is_none")]
    pub completion_cost: Option<f64>,
}

impl Run {
    pub fn new(name: String, run_type: RunType, inputs: Value) -> Self {
        let id = Uuid::new_v4();
        let start_time = Utc::now();

        Self {
            id,
            name,
            run_type,
            inputs,
            outputs: None,
            start_time,
            end_time: None,
            parent_run_id: None,
            trace_id: None,
            dotted_order: None,
            session_id: None,
            session_name: None,
            thread_id: None,
            error: None,
            tags: Vec::new(),
            extra: HashMap::new(),
            prompt_tokens: None,
            completion_tokens: None,
            total_tokens: None,
            total_cost: None,
            prompt_cost: None,
            completion_cost: None,
        }
    }

    pub fn generate_dotted_order(&self, parent_dotted_order: Option<&str>) -> String {
        // Format: YYYYMMDDTHHMMSS{microseconds}Z{uuid}
        // Example: 20240919T171648521691Z0e01bf50-474d-4536-810f-67d3ee7ea3e7
        let timestamp = self.start_time.format("%Y%m%dT%H%M%S");
        let microseconds = self.start_time.timestamp_subsec_micros();
        let uuid_str = self.id.to_string(); // Full UUID with hyphens
        
        let current_part = format!("{}{:06}Z{}", timestamp, microseconds, uuid_str);
        
        if let Some(parent) = parent_dotted_order {
            format!("{}.{}", parent, current_part)
        } else {
            current_part
        }
    }

    pub fn set_error(&mut self, error: &str) {
        self.error = Some(error.to_string());
    }

    pub fn end(&mut self, outputs: Value) {
        self.outputs = Some(outputs);
        self.end_time = Some(Utc::now());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Value>,
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "prompt_tokens", skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<u64>,
    #[serde(rename = "completion_tokens", skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<u64>,
    #[serde(rename = "total_tokens", skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<u64>,
    #[serde(rename = "total_cost", skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<f64>,
}

impl From<&Run> for RunUpdate {
    fn from(run: &Run) -> Self {
        Self {
            outputs: run.outputs.clone(),
            end_time: run.end_time,
            error: run.error.clone(),
            prompt_tokens: run.prompt_tokens,
            completion_tokens: run.completion_tokens,
            total_tokens: run.total_tokens,
            total_cost: run.total_cost,
        }
    }
}

