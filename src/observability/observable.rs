use crate::observability::observer::Observer;
use serde_json::Value;
use std::sync::{Arc, Mutex};

/// Trait for nodes that can be observed
pub trait Observable: Send + Sync {
    /// Add an observer to this observable
    fn add_observer(&mut self, observer: Arc<dyn Observer>);
    
    /// Notify observers that a node has started
    fn notify_start(&self, node_name: &str, inputs: &Value);
    
    /// Notify observers that a node has completed
    fn notify_end(&self, node_name: &str, outputs: &Value);
    
    /// Notify observers that a node encountered an error
    fn notify_error(&self, node_name: &str, error: &str);
}

/// Default implementation of Observable
pub struct ObservableNode {
    observers: Arc<Mutex<Vec<Arc<dyn Observer>>>>,
}

impl ObservableNode {
    pub fn new() -> Self {
        Self {
            observers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for ObservableNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Observable for ObservableNode {
    fn add_observer(&mut self, observer: Arc<dyn Observer>) {
        self.observers.lock().unwrap().push(observer);
    }

    fn notify_start(&self, node_name: &str, inputs: &Value) {
        let observers = self.observers.lock().unwrap();
        for observer in observers.iter() {
            observer.on_node_start(node_name, inputs);
        }
    }

    fn notify_end(&self, node_name: &str, outputs: &Value) {
        let observers = self.observers.lock().unwrap();
        for observer in observers.iter() {
            observer.on_node_end(node_name, outputs);
        }
    }

    fn notify_error(&self, node_name: &str, error: &str) {
        let observers = self.observers.lock().unwrap();
        for observer in observers.iter() {
            observer.on_node_error(node_name, error);
        }
    }
}

