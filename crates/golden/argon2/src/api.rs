use anyhow::Result;
use argon2::{Algorithm, Argon2, Params, Version};

/// Hash a password with the given salt using Argon2id
///
/// # Arguments
/// * `password` - The password bytes to hash
/// * `salt` - The salt string to use for hashing
/// * `m_cost` - Memory cost in KiB (default: 65536)
/// * `t_cost` - Time cost (default: 3)
/// * `p_cost` - Parallelism (default: 4)
///
/// # Returns
/// A 32-byte hash result
///
/// # Errors
/// Returns an error if the hashing operation fails
pub fn hash_with_params(password: &[u8], salt: &str, m_cost: u32, t_cost: u32, p_cost: u32) -> Result<Vec<u8>> {
    let params = Params::new(m_cost, t_cost, p_cost, None)
        .map_err(|e| anyhow::anyhow!("Failed to create Argon2 parameters: {}", e))?;

    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut hash = [0u8; 32];
    argon
        .hash_password_into(password, salt.as_bytes(), &mut hash)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?;

    Ok(hash.to_vec())
}

/// Hash a password with the given salt using default Argon2id parameters
///
/// # Arguments
/// * `password` - The password bytes to hash
/// * `salt` - The salt string to use for hashing
///
/// # Returns
/// A 32-byte hash result
///
/// # Errors
/// Returns an error if the hashing operation fails
pub fn hash(password: &[u8], salt: &str) -> Result<Vec<u8>> {
    hash_with_params(
        password,
        salt,
        Params::DEFAULT_M_COST,
        Params::DEFAULT_T_COST,
        Params::DEFAULT_P_COST,
    )
}

pub fn must_hash<P: AsRef<[u8]>>(password: P, salt: &str) -> u8 {
    hash(password.as_ref(), salt).expect("hash")[0]
}
