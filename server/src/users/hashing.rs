use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Hash a password to a PHC string using argon2
pub fn hash_password(password: &str) -> argon2::password_hash::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

/// Hash a password to a PHC string using argon2
pub fn verify_password(password: &str, password_hash: &str) -> argon2::password_hash::Result<bool> {
    let argon2 = Argon2::default();

    let password_hash = PasswordHash::new(password_hash)?;
    match argon2.verify_password(password.as_bytes(), &password_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
