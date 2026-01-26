use super::*;

#[test]
fn test_wasmtime_v21_creation() {
    let engine = WasmtimeV21::new();
    assert_eq!(engine.version(), "wasmtime-21.0");
}
