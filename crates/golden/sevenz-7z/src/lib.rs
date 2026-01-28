#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

use anyhow::{Context, Result};
use sevenz_rust::{
    Password, SevenZArchiveEntry, SevenZMethod, SevenZMethodConfiguration, SevenZReader,
    SevenZWriter,
};
use std::io::Cursor;

/// Compress the given bytes using LZMA2 algorithm (7z native compression) via sevenz-rust library
///
/// # Arguments
/// * `data` - The byte slice to be compressed
/// * `compression_level` - Compression level (0-9), where 0 is no compression and 9 is maximum
///
/// # Returns
/// A vector containing the compressed 7z archive data
///
/// # Errors
/// Returns an error if the compression operation fails
pub fn compress_with_lzma2(data: &[u8], compression_level: u8) -> Result<Vec<u8>> {
    // Validate compression level
    if compression_level > 9 {
        return Err(anyhow::anyhow!("Compression level must be between 0 and 9"));
    }

    // Create a buffer to hold the compressed data
    let buffer = Cursor::new(Vec::new());

    // Create a SevenZWriter that writes to the buffer
    let mut sz = SevenZWriter::new(buffer).context("Failed to create 7z writer")?;

    // Set content methods to use LZMA2 compression (7z native algorithm)
    sz.set_content_methods(vec![SevenZMethodConfiguration::new(SevenZMethod::LZMA2)]);

    // Create an archive entry
    let mut entry = SevenZArchiveEntry::new();
    entry.name = "compressed_data.bin".to_string();
    entry.has_stream = true;
    entry.is_directory = false;
    entry.size = data.len() as u64;

    // Add the data as a file to the 7z archive
    sz.push_archive_entry(entry, Some(Cursor::new(data.to_vec())))
        .context("Failed to compress data")?;

    // Finalize and get the compressed bytes
    let buffer = sz.finish().context("Failed to finalize 7z archive")?;

    Ok(buffer.into_inner())
}

/// A convenience function to compress with default compression level (6)
pub fn zip(data: &[u8]) -> Result<Vec<u8>> {
    compress_with_lzma2(data, 6)
}

/// Decompress the given bytes that were compressed using sevenz-rust library (LZMA2)
///
/// # Arguments
/// * `compressed_data` - The byte slice containing the compressed 7z archive data
///
/// # Returns
/// A vector containing the decompressed original data
///
/// # Errors
/// Returns an error if:
/// - The input is not a valid 7z archive
/// - The archive contains no files
/// - Failed to read the decompressed data
pub fn unzip(compressed_data: &[u8]) -> Result<Vec<u8>> {
    // Create a cursor to read the compressed data
    let cursor = Cursor::new(compressed_data);

    // Create a SevenZReader
    let mut archive = SevenZReader::new(cursor, compressed_data.len() as u64, Password::empty())
        .context("Failed to open 7z archive")?;

    // Collect the first entry's data
    let mut result: Option<Vec<u8>> = None;

    archive
        .for_each_entries(|_entry, reader| {
            if result.is_none() {
                let mut data = Vec::new();
                reader.read_to_end(&mut data)?;
                result = Some(data);
            }
            Ok::<_, sevenz_rust::Error>(result.is_some())
        })
        .context("Failed to read archive entries")?;

    result.ok_or_else(|| anyhow::anyhow!("7z archive contains no files"))
}

#[cfg(test)]
mod tests;
