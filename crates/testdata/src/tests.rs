use super::*;

#[test]
fn test_load_simple_fixture() {
    let wasm_bytes = load_fixture("simple");
    assert!(!wasm_bytes.is_empty());

    // Validate WASM format using anyhow error handling
    assert!(validate_wasm(&wasm_bytes).is_ok(), "Invalid WASM format");
}

#[test]
fn test_load_complex_fixture() {
    let wasm_bytes = load_fixture("complex");
    assert!(!wasm_bytes.is_empty());

    // Validate WASM format using anyhow error handling
    assert!(validate_wasm(&wasm_bytes).is_ok(), "Invalid WASM format");
}

#[test]
fn test_validate_wasm_empty() {
    let empty_bytes = vec![];
    let result = validate_wasm(&empty_bytes);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "WASM 模块为空");
}

#[test]
fn test_validate_wasm_invalid_magic() {
    let invalid_bytes = vec![0x00, 0x00, 0x00, 0x00];
    let result = validate_wasm(&invalid_bytes);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "无效的 WASM 魔术数字");
}
