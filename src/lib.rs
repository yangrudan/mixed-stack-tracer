//! mixed-stack-tracer: minimal crate exposing merge functionality for prototype/testing.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub mod stack_tracer;

// Conditionally compile Python bindings
#[cfg(feature = "python")]
pub mod python_bindings;

/// Public re-exports for convenience
pub use crate::stack_tracer::SignalTracer;

/// A simple value type for storing Python frame locals
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Int(i64),
    Float(String), // Store as string to avoid float equality issues
    Bool(bool),
    None,
}

/// CallFrame model compatible with probing repository.
/// Supports both native (C/C++) and Python frames.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallFrame {
    CFrame {
        ip: String,
        file: String,
        func: String,
        lineno: i64,
    },
    PyFrame {
        file: String,
        func: String,
        lineno: i64,
        #[serde(default)]
        locals: HashMap<String, Value>,
    },
}