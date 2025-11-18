```markdown
# mixed-stack-tracer

Prototype repository for merging native (C/C++) and Python call stacks.

This project provides:
- A simple CallFrame model (CFrame / PyFrame).
- A robust `merge_python_native_stacks` implementation that:
  - Uses substring matching to detect Python interpreter boundary frames (e.g. `PyEval_EvalFrame*`),
  - Only increments python-frame index when a python frame was successfully consumed,
  - Preserves native frames when Python frames are exhausted,
  - Appends remaining Python frames after processing native stack to avoid losing information.
- Unit tests for the merge logic.
- CI workflow to run `cargo test`.

Usage:
- Clone the repository, run `cargo test`.
- This is a minimal prototype: production usage should integrate real symbol resolution (demangling), an async-signal-safe native collector, and a safe Python stack retrieval routine (requiring GIL and thread-aware access).

Next steps:
- Integrate actual native backtrace gathering and resolution.
- Implement `get_python_stacks(tid)` using CPython C API or a cooperating thread.
- Add integration tests that spawn C->Python->C call chains.

License: MIT
```