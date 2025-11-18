//! mixed-stack-tracer: minimal crate exposing merge functionality for prototype/testing.

pub mod stack_tracer;

/// Public re-exports for convenience
pub use crate::stack_tracer::SignalTracer;

/// A simple CallFrame model used in tests and examples.
/// In real integration this would come from symbol resolution/demangling and probing_proto.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CallFrame {
    CFrame {
        ip: String,
        file: String,
        func: String,
        lineno: i64,
    },
    PyFrame {
        ip: String,
        file: String,
        func: String,
        lineno: i64,
    },
}