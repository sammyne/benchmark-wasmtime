use super::*;

#[test]
fn test_engine_error_type() {
    // Test that EngineError can be created from any error
    let error: EngineError = anyhow::anyhow!("Test error");
    assert_eq!(error.to_string(), "Test error");
}
