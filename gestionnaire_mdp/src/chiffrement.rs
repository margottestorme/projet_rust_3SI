use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};
use base64::{engine::general_purpose, Engine as _};

const NONCE_SIZE: usize = 12;
const KEY_SIZE: usize = 32;

/// Génère une clé AES-256 aléatoire
pub fn generer_cle() -> [u8; KEY_SIZE] {
    let mut cle = [0u8; KEY_SIZE];
    SystemRandom::new().fill(&mut cle).unwrap();
    cle
}

/// Chiffre une donnée avec AES-256-GCM
pub fn chiffrer_donnee(cle: &[u8], texte: &str) -> (String, String) {
    let mut nonce = [0u8; NONCE_SIZE];
    SystemRandom::new().fill(&mut nonce).unwrap();

    let unbound_key = UnboundKey::new(&AES_256_GCM, cle).unwrap();
    let cle = LessSafeKey::new(unbound_key);

    let mut data = texte.as_bytes().to_vec();
    let tag = cle
        .seal_in_place_separate_tag(
            Nonce::assume_unique_for_key(nonce),
            Aad::empty(),
            &mut data,
        )
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

    let decrypted_data = cle.open_in_place(nonce, Aad::empty(), &mut donnees_chiffrees).ok()?;
    Some(String::from_utf8_lossy(decrypted_data).to_string())
}