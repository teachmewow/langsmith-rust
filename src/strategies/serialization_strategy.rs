use serde::Serialize;
use serde_json::Value;

/// Strategy for serialization approaches
pub trait SerializationStrategy: Send + Sync {
    /// Serialize inputs ensuring they're always an object
    fn serialize_inputs<T: Serialize>(&self, value: &T) -> Result<Value, serde_json::Error>;
    
    /// Serialize outputs ensuring they're always an object
    fn serialize_outputs<T: Serialize>(&self, value: &T) -> Result<Value, serde_json::Error>;
}

/// Default serialization strategy (wraps primitives in objects)
pub struct DefaultSerializationStrategy {
    input_key: String,
    output_key: String,
}

impl DefaultSerializationStrategy {
    pub fn new() -> Self {
        Self {
            input_key: "input".to_string(),
            output_key: "output".to_string(),
        }
    }

    pub fn with_keys(input_key: String, output_key: String) -> Self {
        Self {
            input_key,
            output_key,
        }
    }
}

impl Default for DefaultSerializationStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl SerializationStrategy for DefaultSerializationStrategy {
    fn serialize_inputs<T: Serialize>(&self, value: &T) -> Result<Value, serde_json::Error> {
        let mut json_value = serde_json::to_value(value)?;
        if !json_value.is_object() {
            let key = self.input_key.clone();
            json_value = serde_json::json!({ key: json_value });
        }
        Ok(json_value)
    }

    fn serialize_outputs<T: Serialize>(&self, value: &T) -> Result<Value, serde_json::Error> {
        let mut json_value = serde_json::to_value(value)?;
        if !json_value.is_object() {
            let key = self.output_key.clone();
            json_value = serde_json::json!({ key: json_value });
        }
        Ok(json_value)
    }
}

