use langsmith_rust::strategies::serialization_strategy::{SerializationStrategy, DefaultSerializationStrategy};
use serde_json::json;

#[test]
fn test_serialization_strategy_wraps_primitive() {
    let strategy = DefaultSerializationStrategy::new();
    
    let input_value = strategy.serialize_inputs(&"test".to_string()).unwrap();
    assert!(input_value.is_object());
    assert_eq!(input_value["input"], json!("test"));
    
    let output_value = strategy.serialize_outputs(&42).unwrap();
    assert!(output_value.is_object());
    assert_eq!(output_value["output"], json!(42));
}

#[test]
fn test_serialization_strategy_preserves_object() {
    let strategy = DefaultSerializationStrategy::new();
    
    let input_obj = json!({"key": "value"});
    let result = strategy.serialize_inputs(&input_obj).unwrap();
    
    assert_eq!(result, input_obj);
}

#[test]
fn test_serialization_strategy_custom_keys() {
    let strategy = DefaultSerializationStrategy::with_keys(
        "data".to_string(),
        "result".to_string(),
    );
    
    let input_value = strategy.serialize_inputs(&"test".to_string()).unwrap();
    assert_eq!(input_value["data"], json!("test"));
    
    let output_value = strategy.serialize_outputs(&"result".to_string()).unwrap();
    assert_eq!(output_value["result"], json!("result"));
}

