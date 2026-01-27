use super::*;

#[test]
fn test_engine_error_type() {
    // Test that EngineError can be created from any error
    let error: EngineError = anyhow::anyhow!("Test error");
    assert_eq!(error.to_string(), "Test error");
}

#[test]
fn test_v41_wasi_reexport() {
    // Test that wasi module is re-exported in v41
    // This test verifies that the public API exists at compile time
    use crate::v41::wasi;
    
    // The presence of the use statement verifies that the wasi module
    // is properly re-exported from wasmtime-wasi-v41
    let _ = "wasi module is accessible through engine::v41::wasi";
}

#[test]
fn test_v21_wasi_reexport() {
    // Test that wasi module is re-exported in v21
    use crate::v21::wasi;
    
    // The presence of the use statement verifies that the wasi module
    // is properly re-exported from wasmtime-wasi-v21
    let _ = "wasi module is accessible through engine::v21::wasi";
}
