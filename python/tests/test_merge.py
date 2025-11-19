"""
Test cases for mixed-stack-tracer Python bindings.
Can be run directly with Python after installing the package.
"""

import sys
import os

# Try to import the module
try:
    import mixed_stack_tracer
    print("✓ Successfully imported mixed_stack_tracer module")
except ImportError as e:
    print(f"✗ Failed to import module: {e}")
    print("\nTo run these tests, first build and install the Python package:")
    print("  pip install maturin")
    print("  maturin develop --features python")
    sys.exit(1)


def test_create_frames():
    """Test creating CFrame and PyFrame objects"""
    print("\n=== Test: Creating frames ===")
    
    # Create a CFrame
    cframe = mixed_stack_tracer.create_cframe(
        ip="0x12345678",
        file="test.c",
        func="main",
        lineno=42
    )
    print(f"CFrame: {cframe}")
    assert cframe['type'] == 'CFrame'
    assert cframe['func'] == 'main'
    assert cframe['lineno'] == 42
    print("✓ CFrame creation works")
    
    # Create a PyFrame without locals
    pyframe = mixed_stack_tracer.create_pyframe(
        file="test.py",
        func="my_function",
        lineno=10,
        locals=None
    )
    print(f"PyFrame (no locals): {pyframe}")
    assert pyframe['type'] == 'PyFrame'
    assert pyframe['func'] == 'my_function'
    assert pyframe['lineno'] == 10
    assert pyframe['locals'] == {}
    print("✓ PyFrame creation (no locals) works")
    
    # Create a PyFrame with locals
    pyframe_with_locals = mixed_stack_tracer.create_pyframe(
        file="test.py",
        func="another_function",
        lineno=20,
        locals={'x': 42, 'name': 'test', 'flag': True}
    )
    print(f"PyFrame (with locals): {pyframe_with_locals}")
    assert pyframe_with_locals['type'] == 'PyFrame'
    assert pyframe_with_locals['locals']['x'] == 42
    assert pyframe_with_locals['locals']['name'] == 'test'
    assert pyframe_with_locals['locals']['flag'] == True
    print("✓ PyFrame creation (with locals) works")


def test_simple_merge():
    """Test simple merging of Python and native stacks"""
    print("\n=== Test: Simple merge ===")
    
    native_stacks = [
        mixed_stack_tracer.create_cframe("0x1", "", "A", 0),
        mixed_stack_tracer.create_cframe("0x2", "", "PyEval_EvalFrameDefault", 0),
        mixed_stack_tracer.create_cframe("0x3", "", "B", 0),
    ]
    
    python_stacks = [
        mixed_stack_tracer.create_pyframe("", "py1", 0, None),
        mixed_stack_tracer.create_pyframe("", "py2", 0, None),
    ]
    
    merged = mixed_stack_tracer.merge_python_native_stacks(python_stacks, native_stacks)
    
    funcs = [frame['func'] for frame in merged]
    print(f"Merged functions: {funcs}")
    
    expected = ["A", "py1", "B", "py2"]
    assert funcs == expected, f"Expected {expected}, got {funcs}"
    print("✓ Simple merge works correctly")


def test_python_shortage():
    """Test merging when Python frames are fewer than PyEval markers"""
    print("\n=== Test: Python shortage ===")
    
    native_stacks = [
        mixed_stack_tracer.create_cframe("0x1", "", "PyEval_EvalFrameDefault", 0),
        mixed_stack_tracer.create_cframe("0x2", "", "PyEval_EvalFrameDefault", 0),
        mixed_stack_tracer.create_cframe("0x3", "", "C", 0),
    ]
    
    python_stacks = [
        mixed_stack_tracer.create_pyframe("", "py1", 0, None),
    ]
    
    merged = mixed_stack_tracer.merge_python_native_stacks(python_stacks, native_stacks)
    
    funcs = [frame['func'] for frame in merged]
    print(f"Merged functions: {funcs}")
    
    expected = ["py1", "PyEval_EvalFrameDefault", "C"]
    assert funcs == expected, f"Expected {expected}, got {funcs}"
    print("✓ Python shortage handled correctly")


def test_python_extra():
    """Test merging when Python frames are more than PyEval markers"""
    print("\n=== Test: Python extra ===")
    
    native_stacks = [
        mixed_stack_tracer.create_cframe("0x1", "", "A", 0),
        mixed_stack_tracer.create_cframe("0x2", "", "B", 0),
    ]
    
    python_stacks = [
        mixed_stack_tracer.create_pyframe("", "py1", 0, None),
        mixed_stack_tracer.create_pyframe("", "py2", 0, None),
    ]
    
    merged = mixed_stack_tracer.merge_python_native_stacks(python_stacks, native_stacks)
    
    funcs = [frame['func'] for frame in merged]
    print(f"Merged functions: {funcs}")
    
    expected = ["A", "B", "py1", "py2"]
    assert funcs == expected, f"Expected {expected}, got {funcs}"
    print("✓ Extra Python frames appended correctly")


def test_no_python_frames():
    """Test merging with no Python frames"""
    print("\n=== Test: No Python frames ===")
    
    native_stacks = [
        mixed_stack_tracer.create_cframe("0x1", "", "X", 0),
        mixed_stack_tracer.create_cframe("0x2", "", "PyEval_EvalFrameDefault", 0),
        mixed_stack_tracer.create_cframe("0x3", "", "Y", 0),
    ]
    
    python_stacks = []
    
    merged = mixed_stack_tracer.merge_python_native_stacks(python_stacks, native_stacks)
    
    funcs = [frame['func'] for frame in merged]
    print(f"Merged functions: {funcs}")
    
    expected = ["X", "PyEval_EvalFrameDefault", "Y"]
    assert funcs == expected, f"Expected {expected}, got {funcs}"
    print("✓ No Python frames handled correctly")


def main():
    """Run all tests"""
    print("=" * 60)
    print("Running mixed-stack-tracer Python Tests")
    print("=" * 60)
    
    tests = [
        test_create_frames,
        test_simple_merge,
        test_python_shortage,
        test_python_extra,
        test_no_python_frames,
    ]
    
    passed = 0
    failed = 0
    
    for test in tests:
        try:
            test()
            passed += 1
        except AssertionError as e:
            print(f"\n✗ Test failed: {e}")
            failed += 1
        except Exception as e:
            print(f"\n✗ Test error: {e}")
            import traceback
            traceback.print_exc()
            failed += 1
    
    print("\n" + "=" * 60)
    print(f"Test Results: {passed} passed, {failed} failed")
    print("=" * 60)
    
    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
