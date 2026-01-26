use super::*;

#[test]
fn test_wasmtime_v41_creation() {
    let engine = WasmtimeV41::new();
    assert_eq!(engine.version(), "wasmtime-41.0");
}
