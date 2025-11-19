//! Python bindings for mixed-stack-tracer
//! 
//! This module provides PyO3 bindings to use the stack tracer from Python.

use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::{CallFrame, SignalTracer, Value};

/// Convert Rust CallFrame to Python dictionary
fn callframe_to_pydict(py: Python<'_>, frame: &CallFrame) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    match frame {
        CallFrame::CFrame { ip, file, func, lineno } => {
            dict.set_item("type", "CFrame")?;
            dict.set_item("ip", ip)?;
            dict.set_item("file", file)?;
            dict.set_item("func", func)?;
            dict.set_item("lineno", lineno)?;
        }
        CallFrame::PyFrame { file, func, lineno, locals } => {
            dict.set_item("type", "PyFrame")?;
            dict.set_item("file", file)?;
            dict.set_item("func", func)?;
            dict.set_item("lineno", lineno)?;
            
            let locals_dict = PyDict::new(py);
            for (k, v) in locals {
                let py_value = match v {
                    Value::String(s) => s.to_object(py),
                    Value::Int(i) => i.to_object(py),
                    Value::Float(f) => f.to_object(py),
                    Value::Bool(b) => b.to_object(py),
                    Value::None => py.None(),
                };
                locals_dict.set_item(k, py_value)?;
            }
            dict.set_item("locals", locals_dict)?;
        }
    }
    Ok(dict.to_object(py))
}

/// Convert Python dictionary to Rust CallFrame
fn pydict_to_callframe(dict: &PyDict) -> PyResult<CallFrame> {
    let frame_type: String = dict.get_item("type")?.unwrap().extract()?;
    
    match frame_type.as_str() {
        "CFrame" => {
            Ok(CallFrame::CFrame {
                ip: dict.get_item("ip")?.unwrap().extract()?,
                file: dict.get_item("file")?.unwrap().extract()?,
                func: dict.get_item("func")?.unwrap().extract()?,
                lineno: dict.get_item("lineno")?.unwrap().extract()?,
            })
        }
        "PyFrame" => {
            let locals_dict = dict.get_item("locals")?;
            let mut locals = HashMap::new();
            
            if let Some(locals_dict) = locals_dict {
                let locals_dict: &PyDict = locals_dict.downcast()?;
                for (key, value) in locals_dict.iter() {
                    let key_str: String = key.extract()?;
                    let val = if value.is_instance_of::<pyo3::types::PyString>() {
                        Value::String(value.extract()?)
                    } else if value.is_instance_of::<pyo3::types::PyInt>() {
                        Value::Int(value.extract()?)
                    } else if value.is_instance_of::<pyo3::types::PyBool>() {
                        Value::Bool(value.extract()?)
                    } else if value.is_none() {
                        Value::None
                    } else {
                        Value::String(value.str()?.to_string())
                    };
                    locals.insert(key_str, val);
                }
            }
            
            Ok(CallFrame::PyFrame {
                file: dict.get_item("file")?.unwrap().extract()?,
                func: dict.get_item("func")?.unwrap().extract()?,
                lineno: dict.get_item("lineno")?.unwrap().extract()?,
                locals,
            })
        }
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Unknown frame type: {}", frame_type)
        ))
    }
}

/// Merge Python and native stacks
/// 
/// Args:
///     python_stacks: List of Python frame dictionaries
///     native_stacks: List of native frame dictionaries
/// 
/// Returns:
///     List of merged frame dictionaries
#[pyfunction]
fn merge_python_native_stacks(
    py: Python<'_>,
    python_stacks: Vec<&PyDict>,
    native_stacks: Vec<&PyDict>,
) -> PyResult<Vec<PyObject>> {
    let python_frames: Result<Vec<CallFrame>, _> = python_stacks
        .iter()
        .map(|d| pydict_to_callframe(d))
        .collect();
    let python_frames = python_frames?;
    
    let native_frames: Result<Vec<CallFrame>, _> = native_stacks
        .iter()
        .map(|d| pydict_to_callframe(d))
        .collect();
    let native_frames = native_frames?;
    
    let merged = SignalTracer::merge_python_native_stacks(python_frames, native_frames);
    
    merged
        .iter()
        .map(|frame| callframe_to_pydict(py, frame))
        .collect()
}

/// Create a CFrame dictionary
/// 
/// Args:
///     ip: Instruction pointer
///     file: Source file name
///     func: Function name
///     lineno: Line number
/// 
/// Returns:
///     Dictionary representing a CFrame
#[pyfunction]
fn create_cframe(
    py: Python<'_>,
    ip: String,
    file: String,
    func: String,
    lineno: i64,
) -> PyResult<PyObject> {
    let frame = CallFrame::CFrame { ip, file, func, lineno };
    callframe_to_pydict(py, &frame)
}

/// Create a PyFrame dictionary
/// 
/// Args:
///     file: Source file name
///     func: Function name
///     lineno: Line number
///     locals: Optional dictionary of local variables
/// 
/// Returns:
///     Dictionary representing a PyFrame
#[pyfunction]
fn create_pyframe(
    py: Python<'_>,
    file: String,
    func: String,
    lineno: i64,
    locals: Option<&PyDict>,
) -> PyResult<PyObject> {
    let mut locals_map = HashMap::new();
    
    if let Some(locals_dict) = locals {
        for (key, value) in locals_dict.iter() {
            let key_str: String = key.extract()?;
            let val = if value.is_instance_of::<pyo3::types::PyString>() {
                Value::String(value.extract()?)
            } else if value.is_instance_of::<pyo3::types::PyInt>() {
                Value::Int(value.extract()?)
            } else if value.is_instance_of::<pyo3::types::PyBool>() {
                Value::Bool(value.extract()?)
            } else if value.is_none() {
                Value::None
            } else {
                Value::String(value.str()?.to_string())
            };
            locals_map.insert(key_str, val);
        }
    }
    
    let frame = CallFrame::PyFrame { file, func, lineno, locals: locals_map };
    callframe_to_pydict(py, &frame)
}

/// Python module for mixed-stack-tracer
#[pymodule]
fn mixed_stack_tracer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(merge_python_native_stacks, m)?)?;
    m.add_function(wrap_pyfunction!(create_cframe, m)?)?;
    m.add_function(wrap_pyfunction!(create_pyframe, m)?)?;
    Ok(())
}
