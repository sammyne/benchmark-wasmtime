#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

use anyhow::Result;
use pulldown_cmark::{Parser, html};

/// Parse markdown text into HTML
///
/// # Arguments
/// * `markdown` - The markdown text to parse
///
/// # Returns
/// The HTML string
///
/// # Errors
/// Returns an error if the parsing operation fails
pub fn parse(markdown: &str) -> Result<String> {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Ok(html_output)
}
