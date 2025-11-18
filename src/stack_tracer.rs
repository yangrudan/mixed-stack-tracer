//! Merge logic for Python + native stacks (prototype).
//! Contains tests that validate several merging scenarios.

use crate::CallFrame;

/// SignalTracer with merge function (prototype)
#[derive(Debug)]
pub struct SignalTracer;

impl SignalTracer {
    /// Merge python_stacks into native_stacks using heuristic boundaries (PyEval_*).
    ///
    /// Rules:
    /// - Traverse native_stacks in order; for each frame, detect if it's a Python boundary (e.g., PyEval).
    /// - On Python boundary:
    ///   * if a python frame is available, consume exactly one python frame and push it into merged (advance index)
    ///   * otherwise (no python frame available), keep the native frame to avoid losing native context
    /// - On native frame: push native frame
    /// - After traversal, append any remaining python frames to merged
    pub fn merge_python_native_stacks(
        mut python_stacks: Vec<CallFrame>,
        native_stacks: Vec<CallFrame>,
    ) -> Vec<CallFrame> {
        let mut merged = Vec::with_capacity(native_stacks.len() + python_stacks.len());
        let mut python_frame_index: usize = 0;

        #[derive(Debug)]
        enum MergeType {
            MergeNativeFrame,
            MergePythonFrame,
        }

        // Detect PyEval-like boundaries in a robust manner using substring checks.
        fn get_merge_strategy(frame: &CallFrame) -> MergeType {
            let func = match frame {
                CallFrame::CFrame { func, .. } => func.as_str(),
                CallFrame::PyFrame { func, .. } => func.as_str(),
            };

            let is_py_eval = func.contains("PyEval_EvalFrame")
                || func.contains("PyEval_EvalCode")
                || func.starts_with("PyEval")
                || func.contains("EvalFrameDefault")
                || func.contains("EvalFrameEx");

            if is_py_eval {
                MergeType::MergePythonFrame
            } else {
                MergeType::MergeNativeFrame
            }
        }

        for native_frame in native_stacks.into_iter() {
            match get_merge_strategy(&native_frame) {
                MergeType::MergeNativeFrame => merged.push(native_frame),
                MergeType::MergePythonFrame => {
                    if python_frame_index < python_stacks.len() {
                        let py_frame = python_stacks[python_frame_index].clone();
                        merged.push(py_frame);
                        python_frame_index += 1;
                    } else {
                        // No python frames left: preserve native frame
                        merged.push(native_frame);
                    }
                }
            }
        }

        // Append remaining python frames (avoid dropping extra python frames)
        if python_frame_index < python_stacks.len() {
            merged.extend_from_slice(&python_stacks[python_frame_index..]);
        }

        merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CallFrame;

    fn cframe(name: &str) -> CallFrame {
        CallFrame::CFrame {
            ip: "0x0".to_string(),
            file: "".to_string(),
            func: name.to_string(),
            lineno: 0,
        }
    }

    fn pyframe(name: &str) -> CallFrame {
        CallFrame::PyFrame {
            ip: "0x0".to_string(),
            file: "".to_string(),
            func: name.to_string(),
            lineno: 0,
        }
    }

    fn funcs(frames: &[CallFrame]) -> Vec<String> {
        frames
            .iter()
            .map(|f| match f {
                CallFrame::CFrame { func, .. } => func.clone(),
                CallFrame::PyFrame { func, .. } => func.clone(),
            })
            .collect()
    }

    #[test]
    fn test_simple_insert() {
        // native: A -> PyEval -> B
        // python: py1 -> py2
        let native = vec![cframe("A"), cframe("PyEval_EvalFrameDefault"), cframe("B")];
        let python = vec![pyframe("py1"), pyframe("py2")];

        let merged = SignalTracer::merge_python_native_stacks(python, native);
        let got = funcs(&merged);

        // Expect: A, py1, B, py2
        assert_eq!(got, vec!["A", "py1", "B", "py2"]);
    }

    #[test]
    fn test_python_shortage() {
        // native: PyEval, PyEval, C
        // python: only py1
        let native = vec![
            cframe("PyEval_EvalFrameDefault"),
            cframe("PyEval_EvalFrameDefault"),
            cframe("C"),
        ];
        let python = vec![pyframe("py1")];

        let merged = SignalTracer::merge_python_native_stacks(python, native);
        let got = funcs(&merged);

        // Expect:
        // first PyEval -> py1
        // second PyEval -> no python left, keep native PyEval
        // then C
        assert_eq!(got, vec!["py1", "PyEval_EvalFrameDefault", "C"]);
    }

    #[test]
    fn test_python_extra() {
        // native has no PyEval, python has frames => all python frames appended
        let native = vec![cframe("A"), cframe("B")];
        let python = vec![pyframe("py1"), pyframe("py2")];

        let merged = SignalTracer::merge_python_native_stacks(python, native);
        let got = funcs(&merged);

        // Expect: A, B, py1, py2
        assert_eq!(got, vec!["A", "B", "py1", "py2"]);
    }

    #[test]
    fn test_no_python_frames() {
        // native has PyEval markers, but no python frames at all
        let native = vec![cframe("X"), cframe("PyEval_EvalFrameDefault"), cframe("Y")];
        let python: Vec<CallFrame> = vec![];

        let merged = SignalTracer::merge_python_native_stacks(python, native);
        let got = funcs(&merged);

        // Expect: preserve native PyEval since no python frames to insert
        assert_eq!(got, vec!["X", "PyEval_EvalFrameDefault", "Y"]);
    }
}