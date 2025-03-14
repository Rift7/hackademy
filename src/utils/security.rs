use argon2::{Argon2, PasswordHash, PasswordVerifier, password_hash::SaltString};
use argon2::password_hash::{rand_core::OsRng, PasswordHasher};

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let password_hash = argon.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    if let Ok(parsed_hash) = PasswordHash::new(hashed) {
        let argon = Argon2::default();
        argon.verify_password(password.as_bytes(), &parsed_hash).is_ok()
    } else {
        false
    }
}