use rand::rngs::OsRng;
use x25519_dalek::{PublicKey, SharedSecret, StaticSecret};

pub fn generate_keypair() -> (StaticSecret, PublicKey) {
    let secret_key = StaticSecret::new(OsRng);
    let public_key = PublicKey::from(&secret_key);
    (secret_key, public_key)
}

pub fn generate_shared_secret(
    secret_key: StaticSecret,
    receiver_public_key: PublicKey,
) -> SharedSecret {
    secret_key.diffie_hellman(&receiver_public_key)
}