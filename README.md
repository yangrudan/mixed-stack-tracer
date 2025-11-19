# mixed-stack-tracer

Prototype repository for merging native (C/C++) and Python call stacks.

This project provides:
- A robust CallFrame model (CFrame / PyFrame) compatible with [probing](https://github.com/reiase/probing).
- A robust `merge_python_native_stacks` implementation that:
  - Uses substring matching to detect Python interpreter boundary frames (e.g. `PyEval_EvalFrame*`),
  - Only increments python-frame index when a python frame was successfully consumed,
  - Preserves native frames when Python frames are exhausted,
  - Appends remaining Python frames after processing native stack to avoid losing information.
- Unit tests for the merge logic (both Rust and Python).
- CI workflow to run `cargo test`.
- **Python bindings** supporting Python 3.7+ for cross-language usage.

## Features

- **Rust Library**: Core implementation with zero-cost abstractions
- **Python Bindings**: PyO3-based bindings for Python 3.7+
- **Multiple Python Version Support**: Tested with Python 3.7-3.12
- **Serde Support**: JSON serialization/deserialization for CallFrame structures

## Usage

### Rust Library

Clone the repository and run tests:

```bash
cargo test
```

Use in your Rust project:

```rust
use mixed_stack_tracer::{CallFrame, SignalTracer};

let python_frames = vec![/* ... */];
let native_frames = vec![/* ... */];

let merged = SignalTracer::merge_python_native_stacks(python_frames, native_frames);
```

### Python Bindings

#### Installation

First, install maturin:

```bash
pip install maturin
```

Then build and install the package:

```bash
# Development mode (for testing)
maturin develop --features python

# Or build a wheel for distribution
maturin build --release --features python
pip install target/wheels/mixed_stack_tracer-*.whl
```

#### Running Python Tests

```bash
python python/tests/test_merge.py
```

#### Python API Example

```python
import mixed_stack_tracer

# Create frames
cframe = mixed_stack_tracer.create_cframe(
    ip="0x12345678",
    file="main.c",
    func="main",
    lineno=42
)

pyframe = mixed_stack_tracer.create_pyframe(
    file="app.py",
    func="process_data",
    lineno=100,
    locals={'x': 42, 'name': 'test'}
)

# Merge stacks
native_stacks = [cframe, ...]
python_stacks = [pyframe, ...]

merged = mixed_stack_tracer.merge_python_native_stacks(
    python_stacks,
    native_stacks
)

for frame in merged:
    print(f"{frame['type']}: {frame['func']} at {frame['file']}:{frame['lineno']}")
```

## Data Structures

The `CallFrame` enum supports two types of frames:

### CFrame (Native C/C++ Frame)
```rust
CFrame {
    ip: String,        // instruction pointer
    file: String,      // source file
    func: String,      // function name
    lineno: i64,      // line number
}
```

### PyFrame (Python Frame)
```rust
PyFrame {
    file: String,                      // source file
    func: String,                      // function name
    lineno: i64,                      // line number
    locals: HashMap<String, Value>,   // local variables
}
```

These structures are compatible with the [probing](https://github.com/reiase/probing) repository's CallFrame definition.

## Testing with Multiple Python Versions

The package supports Python 3.7 through 3.12. See [python/tests/README.md](python/tests/README.md) for detailed instructions on testing with different Python versions.

## CI/CD

The project includes GitHub Actions workflows that:
- Run Rust tests with `cargo test`
- Can be extended to test multiple Python versions

## Next Steps

- Integrate actual native backtrace gathering and resolution.
- Implement `get_python_stacks(tid)` using CPython C API or a cooperating thread.
- Add integration tests that spawn C->Python->C call chains.

## License

MIT
