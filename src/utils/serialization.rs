use serde::Serialize;
use serde_json::Value;

/// Ensures that a serialized value is always an object.
/// If the value is a primitive type (string, number, bool, null, array),
/// it wraps it in an object with a default key.
pub fn ensure_object<T: Serialize>(value: T, key: &str) -> Result<Value, serde_json::Error> {
    let mut json_value = serde_json::to_value(value)?;
    
    if !json_value.is_object() {
        json_value = serde_json::json!({ key: json_value });
    }
    
    Ok(json_value)
}

/// Ensures inputs are always an object
pub fn ensure_inputs_object<T: Serialize>(value: T) -> Result<Value, serde_json::Error> {
    ensure_object(value, "input")
}

/// Ensures outputs are always an object
pub fn ensure_outputs_object<T: Serialize>(value: T) -> Result<Value, serde_json::Error> {
    ensure_object(value, "output")
}

