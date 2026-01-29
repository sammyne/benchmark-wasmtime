use anyhow::{Result, bail};

/// Load a WASM fixture by name from the fixtures directory
///
/// # Arguments
/// * `name` - The name of the fixture file (without .wasm extension)
///
/// # Returns
/// The bytes of the loaded WASM module
///
/// # Panics
/// Panics if the fixture file cannot be found or read
pub fn load_fixture(name: &str) -> Vec<u8> {
    // Get the manifest directory of the testdata crate
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = manifest_dir
        .join("src/fixtures")
        .join(format!("{}.wasm", name));

    if !fixture_path.exists() {
        panic!("Fixture file not found: {:?}", fixture_path);
    }

    std::fs::read(&fixture_path).unwrap_or_else(|e| {
        panic!(
            "Failed to load fixture '{}' from path {:?}: {}",
            name, fixture_path, e
        )
    })
}

/// Validate WASM bytes
///
/// # Arguments
/// * `wasm_bytes` - The WASM bytes to validate
///
/// # Returns
/// Ok(()) if the bytes are valid WASM
///
/// # Errors
/// Returns an error if the bytes are not valid WASM
pub fn validate_wasm(wasm_bytes: &[u8]) -> Result<()> {
    if wasm_bytes.is_empty() {
        bail!("WASM 模块为空");
    }

    if !wasm_bytes.starts_with(&[0x00, 0x61, 0x73, 0x6D]) {
        bail!("无效的 WASM 魔术数字");
    }

    Ok(())
}

#[cfg(test)]
mod tests;
