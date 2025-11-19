# Python Tests for mixed-stack-tracer

This directory contains Python test cases that can be run directly to verify the stack merging functionality.

## Prerequisites

You need to have Python (3.7+) and maturin installed:

```bash
pip install maturin
```

## Building and Testing

### Build the Python extension (development mode)

```bash
maturin develop --features python
```

This will build the Rust code and install the Python module in development mode.

### Run the tests

```bash
python python/tests/test_merge.py
```

Or run from the project root:

```bash
cd /path/to/mixed-stack-tracer
python python/tests/test_merge.py
```

## Building for Production

To build a wheel for distribution:

```bash
maturin build --release --features python
```

The wheel will be created in `target/wheels/`.

## Installing the Package

```bash
pip install target/wheels/mixed_stack_tracer-*.whl
```

## Multi-Version Python Testing

To test with different Python versions, you can use:

```bash
# Python 3.8
python3.8 -m venv venv38
source venv38/bin/activate
pip install maturin
maturin develop --features python
python python/tests/test_merge.py
deactivate

# Python 3.9
python3.9 -m venv venv39
source venv39/bin/activate
pip install maturin
maturin develop --features python
python python/tests/test_merge.py
deactivate

# Python 3.10
python3.10 -m venv venv310
source venv310/bin/activate
pip install maturin
maturin develop --features python
python python/tests/test_merge.py
deactivate
```
