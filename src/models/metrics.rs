use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub prompt_tokens: Option<u64>,
    pub completion_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
    pub total_cost: Option<f64>,
    pub prompt_cost: Option<f64>,
    pub completion_cost: Option<f64>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            prompt_tokens: None,
            completion_tokens: None,
            total_tokens: None,
            total_cost: None,
            prompt_cost: None,
            completion_cost: None,
        }
    }
}

impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tokens(mut self, prompt: u64, completion: u64) -> Self {
        self.prompt_tokens = Some(prompt);
        self.completion_tokens = Some(completion);
        self.total_tokens = Some(prompt + completion);
        self
    }

    pub fn with_costs(mut self, prompt: f64, completion: f64) -> Self {
        self.prompt_cost = Some(prompt);
        self.completion_cost = Some(completion);
        self.total_cost = Some(prompt + completion);
        self
    }
}

