# Copilot Instructions for mixed-stack-tracer

## Project Overview

This is a **hybrid Rust/Python library** that merges native (C/C++) and Python call stacks. The core algorithm is in Rust with PyO3 Python bindings for cross-language usage. The project is compatible with the [probing](https://github.com/reiase/probing) repository's CallFrame model.

## Architecture

### Core Components

1. **CallFrame Model** (`src/lib.rs`): Two-variant enum supporting both native and Python frames
   - `CFrame`: Native frames with `{ip, file, func, lineno}`
   - `PyFrame`: Python frames with `{file, func, lineno, locals: HashMap<String, Value>}`
   - Uses `serde` for JSON serialization

2. **Merge Algorithm** (`src/stack_tracer.rs`): The `SignalTracer::merge_python_native_stacks()` function
   - **Key heuristic**: Detects Python interpreter boundary frames by substring matching (`PyEval_EvalFrame*`, `PyEval_EvalCode*`, `EvalFrameDefault`, etc.)
   - **Consumes Python frames one-at-a-time** when a boundary frame is detected
   - **Preserves native frames** when Python frames are exhausted (prevents data loss)
   - **Appends remaining Python frames** after native stack processing

3. **Python Bindings** (`src/python_bindings.rs`): PyO3-based FFI layer
   - Converts between Rust `CallFrame` and Python dictionaries
   - Exposes `merge_python_native_stacks()`, `create_cframe()`, `create_pyframe()` functions

### Build System

This is a **dual-target project** with different build modes:

- **Rust library**: `cargo build` produces `rlib` + `cdylib`
- **Python package**: Uses `maturin` (PyO3 build tool) for Python wheels
- **Feature flag**: `python` feature enables PyO3 bindings (`pyo3/extension-module`)

## Critical Developer Workflows

### Building and Testing Rust

```bash
# Run Rust unit tests (in stack_tracer.rs)
cargo test

# Build debug library
cargo build
```

### Building and Testing Python Bindings

```bash
# Install build tool (one-time)
pip install maturin

# Development build (editable install)
maturin develop --features python

# Run Python tests
python python/tests/test_merge.py

# Run example
python python/example.py

# Production build (wheel output to target/wheels/)
maturin build --release --features python
pip install target/wheels/mixed_stack_tracer-*.whl
```

**Important**: Always use `--features python` when building for Python usage.

### Multi-Version Python Testing

The CI tests Python 3.8-3.12. To test locally with a specific version:

```bash
python3.X -m venv venvX
source venvX/bin/activate
pip install maturin
maturin develop --features python
python python/tests/test_merge.py
deactivate
```

## Key Conventions

### Frame Type Detection Logic

When modifying the merge algorithm, maintain the substring-based detection pattern in `get_merge_strategy()`:

```rust
// These patterns identify Python interpreter boundary frames
func.contains("PyEval_EvalFrame")
    || func.contains("PyEval_EvalCode")
    || func.starts_with("PyEval")
    || func.contains("EvalFrameDefault")
    || func.contains("EvalFrameEx")
```

### Test Structure

- **Rust tests**: Inline in `src/stack_tracer.rs` using helper functions `cframe()`, `pyframe()`, and `funcs()`
- **Python tests**: Standalone in `python/tests/test_merge.py` with parallel test cases
- **Test pattern**: Create frames → merge → assert function name sequence

Example from both test suites:
```rust
// Rust
let native = vec![cframe("A"), cframe("PyEval_EvalFrameDefault"), cframe("B")];
let python = vec![pyframe("py1"), pyframe("py2")];
let merged = SignalTracer::merge_python_native_stacks(python, native);
assert_eq!(funcs(&merged), vec!["A", "py1", "B", "py2"]);
```

```python
# Python (equivalent)
native_stacks = [create_cframe("0x1", "", "A", 0), 
                 create_cframe("0x2", "", "PyEval_EvalFrameDefault", 0),
                 create_cframe("0x3", "", "B", 0)]
python_stacks = [create_pyframe("", "py1", 0, None),
                 create_pyframe("", "py2", 0, None)]
merged = merge_python_native_stacks(python_stacks, native_stacks)
assert [f['func'] for f in merged] == ["A", "py1", "B", "py2"]
```

### Python Bindings Pattern

When extending the Python API:
1. Add Rust function in `python_bindings.rs`
2. Convert between `PyDict` and Rust types using `pydict_to_callframe()` / `callframe_to_pydict()`
3. Register with `#[pyfunction]` and add to `#[pymodule]`
4. Update `python/tests/test_merge.py` with corresponding tests

## Integration Points

- **probing compatibility**: CallFrame structure matches [reiase/probing](https://github.com/reiase/probing) serialization format
- **PyO3 version**: Locked to `0.20` for ABI stability across Python 3.7-3.12
- **Serde JSON**: Used for CallFrame serialization; all frame types must implement `Serialize` + `Deserialize`

## Common Pitfalls

1. **Missing `--features python`**: Without this flag, Python bindings won't compile
2. **Maturin not installed**: Use `pip install maturin`, not `cargo install maturin`
3. **Python frame order**: The algorithm expects `python_stacks[0]` to be the innermost frame (reverse of some profilers)
4. **Float equality in tests**: `Value::Float` stores as string to avoid Rust float comparison issues

## Next Steps (from README)

The project is still a prototype. Planned improvements:
- Integrate actual native backtrace gathering (currently uses mock frames)
- Implement `get_python_stacks(tid)` using CPython C API
- Add integration tests with real C→Python→C call chains
