use argon2::{PasswordVerifier, PasswordHasher, PasswordHash};
use argon2::password_hash::{Error, SaltString};
use argon2::Argon2;
use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::rngs::OsRng;

use crate::models::User;

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn authorize_user(user: &User, credentials: &Credentials) -> Result<String, Error> {
    let db_hash = PasswordHash::new(&user.password)?;
    let argon = Argon2::default();
    argon.verify_password(
        credentials.password.as_bytes(), &db_hash
    )?;

    Ok(rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect())
}

pub fn hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(OsRng);
    let argon = Argon2::default();
    let password_hash = argon.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}
