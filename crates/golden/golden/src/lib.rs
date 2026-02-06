use std::path::PathBuf;
use std::sync::LazyLock;

static GOLDEN_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("get golden dir")
        .to_path_buf()
});

pub const ARGON2: &str = "argon2";
pub const PULLDOWN_CMARK: &str = "pulldown-cmark";
pub const RUST_PYTHON: &str = "rust-python";
pub const SEVENZ_7Z: &str = "sevenz-7z";

pub fn cmd_path(name: &str) -> PathBuf {
    let mut p = GOLDEN_DIR.clone();
    p.push(name);
    p.push("out");
    p.push("cmd.wasm");
    p
}

pub fn reactor_path(name: &str) -> PathBuf {
    let mut p = GOLDEN_DIR.clone();
    p.push(name);
    p.push("out");
    p.push("reactor.wasm");
    p
}
