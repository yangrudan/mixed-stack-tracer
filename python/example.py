#!/usr/bin/env python3
"""
Example usage of mixed-stack-tracer Python bindings

This example demonstrates how to create and merge native and Python call frames.
"""

import mixed_stack_tracer

def main():
    print("=" * 60)
    print("mixed-stack-tracer Example")
    print("=" * 60)
    
    # Example 1: Create a native C frame
    print("\n1. Creating a native C/C++ frame:")
    cframe = mixed_stack_tracer.create_cframe(
        ip="0xdeadbeef",
        file="/usr/src/app/main.c",
        func="process_request",
        lineno=142
    )
    print(f"   CFrame: {cframe['func']} at {cframe['file']}:{cframe['lineno']}")
    
    # Example 2: Create a Python frame with local variables
    print("\n2. Creating a Python frame with locals:")
    pyframe = mixed_stack_tracer.create_pyframe(
        file="/app/server.py",
        func="handle_connection",
        lineno=87,
        locals={
            'client_ip': '192.168.1.100',
            'port': 8080,
            'connected': True,
            'retry_count': 3
        }
    )
    print(f"   PyFrame: {pyframe['func']} at {pyframe['file']}:{pyframe['lineno']}")
    print(f"   Locals: {pyframe['locals']}")
    
    # Example 3: Simulate a mixed call stack
    print("\n3. Simulating a mixed native/Python call stack:")
    print("   Call chain: main() -> PyEval -> app.run() -> PyEval -> process() -> native_func()")
    
    # Native stack captured by signal handler
    native_frames = [
        mixed_stack_tracer.create_cframe("0x400000", "main.c", "main", 100),
        mixed_stack_tracer.create_cframe("0x500000", "pyeval.c", "PyEval_EvalFrameDefault", 200),
        mixed_stack_tracer.create_cframe("0x600000", "pyeval.c", "PyEval_EvalFrameDefault", 250),
        mixed_stack_tracer.create_cframe("0x700000", "native.c", "native_func", 42),
    ]
    
    # Python stack from interpreter
    python_frames = [
        mixed_stack_tracer.create_pyframe("app.py", "run", 10, {'server': 'http://localhost'}),
        mixed_stack_tracer.create_pyframe("process.py", "process", 50, {'data': 'payload'}),
    ]
    
    # Merge the stacks
    merged_frames = mixed_stack_tracer.merge_python_native_stacks(
        python_frames,
        native_frames
    )
    
    # Display merged stack
    print("\n4. Merged call stack:")
    for i, frame in enumerate(merged_frames):
        frame_type = frame['type']
        func = frame['func']
        file = frame['file']
        lineno = frame['lineno']
        
        if frame_type == 'CFrame':
            ip = frame['ip']
            print(f"   #{i:2d} [{frame_type:7s}] {func:30s} at {file}:{lineno} (ip={ip})")
        else:  # PyFrame
            locals_str = ', '.join([f"{k}={v}" for k, v in frame['locals'].items()]) if frame['locals'] else "no locals"
            print(f"   #{i:2d} [{frame_type:7s}] {func:30s} at {file}:{lineno}")
            if frame['locals']:
                print(f"       └─ locals: {locals_str}")
    
    print("\n" + "=" * 60)
    print("✓ Example completed successfully!")
    print("=" * 60)

if __name__ == "__main__":
    main()
