use argon2::{hash_encoded, verify_encoded, Config};
use rand::Rng;

// hash is from the database and credential is password from the user input.
pub fn hash(credential: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    hash_encoded(credential, &salt, &config).unwrap()
}

pub fn verify(hash: &str, credential: &[u8]) -> bool {
    verify_encoded(hash, credential).unwrap_or(false)
}
