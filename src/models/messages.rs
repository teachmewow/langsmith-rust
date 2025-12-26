use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub args: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMessage {
    pub content: String,
    #[serde(rename = "tool_calls", skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCall>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMessage {
    #[serde(rename = "tool_call_id")]
    pub tool_call_id: String,
    pub content: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanMessage {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMessage {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum Message {
    #[serde(rename = "ai")]
    AI(AIMessage),
    #[serde(rename = "tool")]
    Tool(ToolMessage),
    #[serde(rename = "human")]
    Human(HumanMessage),
    #[serde(rename = "system")]
    System(SystemMessage),
}

