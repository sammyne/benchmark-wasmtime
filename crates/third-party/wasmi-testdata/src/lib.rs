// Include wasm files from wasmi benchmarks
// Source: https://github.com/wasmi-labs/wasmi-benchmarks
// Commit: b9385cae9bfb8cf84dbb13996d0b948ca5826b53

/// Argon2 password hashing algorithm
pub const ARGON2: &[u8] = include_bytes!("../static/argon2.wasm");

/// bzip2 compression algorithm
pub const BZ2: &[u8] = include_bytes!("../static/bz2.wasm");

/// CoreMark benchmark (minimal version)
pub const COREMARK_MINIMAL: &[u8] = include_bytes!("../static/coremark-minimal.wasm");

/// ERC20 token standard contract
pub const ERC20: &[u8] = include_bytes!("../static/erc20.wasm");

/// FFmpeg video processing library
pub const FFMPEG: &[u8] = include_bytes!("../static/ffmpeg.wasm");

/// Markdown parser (CommonMark compliant)
pub const PULLDOWN_CMARK: &[u8] = include_bytes!("../static/pulldown-cmark.wasm");

/// SpiderMonkey JavaScript engine
pub const SPIDERMONKEY: &[u8] = include_bytes!("../static/spidermonkey.wasm");
