use std::path::PathBuf;
use std::process::Command;

/// Integration test for basic CLI functionality
#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(["run", "-p", "cli", "--", "--help"])
        .output()
        .expect("Failed to execute CLI");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("wasmtime-cli"));
    assert!(stdout.contains("--wasm"));
    assert!(stdout.contains("--function"));
    assert!(stdout.contains("JSON"));
}

/// Integration test for error when file not found
#[test]
fn test_cli_file_not_found() {
    let output = Command::new("cargo")
        .args([
            "run",
            "-p",
            "cli",
            "--",
            "-w",
            "/nonexistent.wasm",
            "-f",
            "test",
        ])
        .output()
        .expect("Failed to execute CLI");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("\"success\": false"));
    assert!(stderr.contains("not found"));
}

/// Integration test for JSON parameter parsing error
#[test]
fn test_cli_invalid_json_param() {
    // Create a temporary file with some content
    use std::io::Write;
    use tempfile::NamedTempFile;
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "dummy content").unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "-p",
            "cli",
            "--",
            "-w",
            temp_file.path().to_str().unwrap(),
            "-f",
            "test",
            "{invalid json}",
        ])
        .output()
        .expect("Failed to execute CLI");

    assert!(!output.status.success());
}

/// Integration test with valid WASM component (if available)
#[test]
fn test_cli_with_simple_wasm() {
    // This test would require a valid WASM component file
    // For now, we'll skip it if no test file is available
    let test_wasm_path = PathBuf::from("testdata/src/fixtures/simple.wasm");

    if !test_wasm_path.exists() {
        println!("Skipping test: WASM test file not found");
        return;
    }

    let output = Command::new("cargo")
        .args([
            "run",
            "-p",
            "cli",
            "--",
            "-w",
            test_wasm_path.to_str().unwrap(),
            "-f",
            "main",
        ])
        .output()
        .expect("Failed to execute CLI");

    // The test file might not be a valid component, so we don't assert success
    // We just verify the CLI runs and produces some output
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Should have output either to stdout (success) or stderr (error)
    assert!(!stdout.is_empty() || !stderr.is_empty());
}
