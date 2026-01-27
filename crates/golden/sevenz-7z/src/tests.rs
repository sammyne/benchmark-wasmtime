use super::*;

// Define test case structure for table-driven tests
struct Case {
    name: &'static str,
    input: Vec<u8>,
    compression_level: u8,
}

fn test_vector() -> Vec<Case> {
    vec![
        Case {
            name: "empty_data",
            input: vec![],
            compression_level: 6,
        },
        Case {
            name: "small_data",
            input: vec![1, 2, 3, 4, 5],
            compression_level: 6,
        },
        Case {
            name: "repetitive_data",
            input: vec![42; 1000],
            compression_level: 9,
        },
        Case {
            name: "random_data",
            input: (0..100).map(|i| (i % 256) as u8).collect(),
            compression_level: 3,
        },
        Case {
            name: "minimum_compression",
            input: vec![1, 2, 3, 4, 5],
            compression_level: 1,
        },
        Case {
            name: "maximum_compression",
            input: vec![0; 1000],
            compression_level: 9,
        },
    ]
}

#[test]
fn compress_with_all_levels() {
    for case in test_vector() {
        let result = compress_with_lzma2(&case.input, case.compression_level);
        assert!(
            result.is_ok(),
            "Test case '{}' failed: {:?}",
            case.name,
            result.err()
        );

        let compressed = result.unwrap();
        // Verify that we got some output
        assert!(
            !compressed.is_empty(),
            "Test case '{}' produced empty output",
            case.name
        );
    }
}

#[test]
fn compress_default_level() {
    let data = vec![1, 2, 3, 4, 5];
    let result = zip(&data);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
}

#[test]
fn invalid_level_rejected() {
    let data = vec![1, 2, 3];
    let result = compress_with_lzma2(&data, 10);
    assert!(result.is_err());
}

#[test]
fn compression_reduces_size() {
    // Highly compressible data
    let data = vec![42; 10000];
    let compressed = compress_with_lzma2(&data, 9).unwrap();

    // Compressed data should be significantly smaller
    assert!(
        compressed.len() < data.len(),
        "Compressed size {} should be less than original size {}",
        compressed.len(),
        data.len()
    );
}

#[test]
fn empty_input_handling() {
    let result = zip(&[]);
    assert!(result.is_ok());
    let compressed = result.unwrap();
    // Even empty data should produce a valid 7z archive (with headers)
    assert!(!compressed.is_empty());
}

// Decompression tests

// Define decompression test case structure
struct DecompressCase {
    name: &'static str,
    input: Vec<u8>,
}

fn decompress_test_vector() -> Vec<DecompressCase> {
    vec![
        DecompressCase {
            name: "empty_data",
            input: vec![],
        },
        DecompressCase {
            name: "small_data",
            input: vec![1, 2, 3, 4, 5],
        },
        DecompressCase {
            name: "repetitive_data",
            input: vec![42; 1000],
        },
        DecompressCase {
            name: "random_data",
            input: (0..100).map(|i| (i % 256) as u8).collect(),
        },
        DecompressCase {
            name: "binary_data",
            input: vec![
                0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0x00, 0x01, 0x7F, 0x80, 0x9A, 0xBC,
            ],
        },
    ]
}

#[test]
fn decompress_roundtrip() {
    for case in decompress_test_vector() {
        // Compress the data
        let compressed = zip(&case.input)
            .context(format!("Failed to compress test case '{}'", case.name))
            .unwrap();

        // Decompress the data
        let decompressed = unzip(&compressed)
            .context(format!("Failed to decompress test case '{}'", case.name))
            .unwrap();

        // Verify the decompressed data matches the original
        assert_eq!(
            decompressed, case.input,
            "Test case '{}' failed: decompressed data does not match original",
            case.name
        );
    }
}

#[test]
fn decompress_invalid_archive() {
    let invalid_data = vec![0, 1, 2, 3, 4, 5];
    let result = unzip(&invalid_data);
    assert!(result.is_err(), "Should fail on invalid 7z archive");
}

#[test]
fn decompress_with_compression_levels() {
    for case in test_vector() {
        // Compress with specific level
        let compressed = compress_with_lzma2(&case.input, case.compression_level)
            .context(format!(
                "Failed to compress '{}' with level {}",
                case.name, case.compression_level
            ))
            .unwrap();

        // Decompress and verify
        let decompressed = unzip(&compressed)
            .context(format!("Failed to decompress '{}'", case.name))
            .unwrap();

        assert_eq!(
            decompressed, case.input,
            "Roundtrip failed for '{}' at compression level {}",
            case.name, case.compression_level
        );
    }
}

#[test]
fn decompress_large_data() {
    // Test with larger data
    let large_data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();

    let compressed = zip(&large_data).unwrap();
    let decompressed = unzip(&compressed).unwrap();

    assert_eq!(decompressed, large_data);
}

#[test]
fn decompress_all_zeros() {
    let data = vec![0u8; 5000];
    let compressed = zip(&data).unwrap();
    let decompressed = unzip(&compressed).unwrap();

    assert_eq!(decompressed, data);
}
