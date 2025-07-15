use base64::{Engine as _, engine::general_purpose};
use ring::aead::{AES_256_GCM, Aad, LessSafeKey, Nonce, UnboundKey};
use ring::{
    pbkdf2,
    rand::{SecureRandom, SystemRandom},
};
use std::num::NonZeroU32;

const NONCE_SIZE: usize = 12;
const CREDENTIAL_LEN: usize = 32;
const ITERATIONS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100_000) };

pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    SystemRandom::new().fill(&mut salt).unwrap();
    salt
}

pub fn derive_key(password: &str, salt: &[u8]) -> [u8; CREDENTIAL_LEN] {
    let mut key = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        ITERATIONS,
        salt,
        password.as_bytes(),
        &mut key,
    );
    key
}

/// Chiffre une donnée avec AES-256-GCM
pub fn chiffrer_donnee(cle: &[u8], texte: &str) -> (String, String) {
    let mut nonce = [0u8; NONCE_SIZE];
    SystemRandom::new().fill(&mut nonce).unwrap();

    let unbound_key = UnboundKey::new(&AES_256_GCM, cle).unwrap();
    let cle = LessSafeKey::new(unbound_key);

    let mut data = texte.as_bytes().to_vec();
    let tag = cle
        .seal_in_place_separate_tag(Nonce::assume_unique_for_key(nonce), Aad::empty(), &mut data)
        .unwrap();

    data.extend_from_slice(tag.as_ref());

    let encrypted_data = general_purpose::STANDARD.encode(data);
    let encoded_nonce = general_purpose::STANDARD.encode(nonce);

    (encrypted_data, encoded_nonce)
}

pub fn dechiffrer_donnee(cle: &[u8], texte_chiffre: &str, nonce_str: &str) -> Option<String> {
    let unbound_key = UnboundKey::new(&AES_256_GCM, cle).ok()?;
    let cle = LessSafeKey::new(unbound_key);

    let nonce_bytes = general_purpose::STANDARD.decode(nonce_str).ok()?;
    let nonce = Nonce::assume_unique_for_key(nonce_bytes.as_slice().try_into().ok()?);

    let mut donnees_chiffrees = general_purpose::STANDARD.decode(texte_chiffre).ok()?;

    let decrypted_data = cle
        .open_in_place(nonce, Aad::empty(), &mut donnees_chiffrees)
        .ok()?;
    Some(String::from_utf8_lossy(decrypted_data).to_string())
}
