extern crate bcrypt;

use bcrypt::{hash, verify, BcryptError};
const DEFAULT_COST: u32 = 8;

pub fn encrypt_password(password: &String) -> Result<String, BcryptError> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn compare_password(
    user_password: &String,
    given_password: &String,
) -> Result<bool, BcryptError> {
    let valid = verify(given_password, user_password)?;
    Ok(valid)
}
