/// Error type for engine operations
pub type EngineError = anyhow::Error;

pub mod v21;
pub mod v41;

#[cfg(test)]
mod tests;
