use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, SaltString
    },
    Argon2, PasswordVerifier,
};

#[tracing::instrument(name = "Hashing user password", skip(password))]
pub async fn hash(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password, &salt)
        .expect("Unable to hash password.")
        .to_string()
}

#[tracing::instrument(name = "Verifying user password", skip(password, hash))]
pub async fn verify_password(
    hash: String,
    password: String,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash.as_str())?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}