// #![cfg_attr(target_arch = "wasm32", no_std)]

// #[cfg(target_arch = "wasm32")]
// #[panic_handler]
// fn handle_panic(_panic_info: &::core::panic::PanicInfo) -> ! {
//     core::arch::wasm32::unreachable()
// }

// #[cfg(target_arch = "wasm32")]
// #[global_allocator]
// static GLOBAL: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

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
pub fn hash_with_params(
    password: &[u8],
    salt: &str,
    m_cost: u32,
    t_cost: u32,
    p_cost: u32,
) -> Result<Vec<u8>> {
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

#[no_mangle]
pub extern "C" fn run(input: i64) -> i64 {
    if !(0..=10).contains(&input) {
        panic!("invalid input: {input}")
    }
    let password = b"some random password";
    let salt = b"some random salt";
    let m_cost = (input as u32) * 1024;
    let t_cost = argon2::Params::DEFAULT_T_COST;
    let p_cost = argon2::Params::DEFAULT_P_COST;

    let params = argon2::Params::new(m_cost, t_cost, p_cost, None).unwrap();
    let argon = argon2::Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    let mut hash = [0u8; 32];
    argon.hash_password_into(password, salt, &mut hash).unwrap();
    i64::from_be_bytes(
        <[u8; 8]>::try_from(&hash[..8]).expect("array and slice have the same length"),
    )
}
