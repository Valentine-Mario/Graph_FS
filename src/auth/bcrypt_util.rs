extern crate bcrypt;

use bcrypt::{hash, verify, BcryptError};
const DEFAULT_COST: u32 = 8;

pub fn encrypt_password(password: &str) -> Result<String, BcryptError> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn compare_password(user_password: &str, given_password: &str) -> Result<bool, BcryptError> {
    let valid = verify(given_password, user_password)?;
    Ok(valid)
}
