//! LangSmith Rust - Manual tracing for LangSmith
//!
//! This crate provides a Rust implementation for manual tracing to LangSmith,
//! similar to the Python and TypeScript SDKs.

// Load .env file on module initialization
fn init_dotenv() {
    let _ = dotenvy::dotenv();
}

// Initialize dotenv when the module is loaded
#[allow(dead_code)]
static INIT: std::sync::Once = std::sync::Once::new();

pub mod client;
pub mod config;
pub mod error;
pub mod factories;
pub mod models;
pub mod observability;
pub mod strategies;
pub mod tracing;
pub mod utils;

// Re-export main types
pub use client::LangSmithClient;
pub use config::Config;
pub use error::{LangSmithError, Result};
pub use factories::TracerFactory;
pub use models::{
    metrics::Metrics,
    AIMessage, HumanMessage, Message, Run, RunType, RunUpdate, SystemMessage, ToolCall,
    ToolMessage,
};
pub use observability::{LangSmithObserver, Observable, ObservableNodeWrapper, Observer};
pub use strategies::{SerializationStrategy, TracingStrategy};
pub use tracing::{trace_node, trace_node_sync, GraphTrace, RunScope, TraceContext, Tracer};

// Initialize dotenv on first use
pub fn init() {
    INIT.call_once(|| {
        init_dotenv();
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_compiles() {
        // Verify the module compiles
        assert!(true);
    }
}
