use wasm_bindgen::prelude::*;
use rsa::{PublicKey, RSAPublicKey, RSAPrivateKey, PaddingScheme};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct RSAKeyPair {
  rsa_private_key: RSAPrivateKey,
  rsa_public_key: RSAPublicKey
}


#[wasm_bindgen]
pub fn generate_key_pair (bits: i32) -> String {
  let mut rng = OsRng;
  let private_key = RSAPrivateKey::new(&mut rng, bits as usize).expect("failed to generate a key");
  let public_key = private_key.to_public_key();
  let key_pair = RSAKeyPair {rsa_private_key: private_key, rsa_public_key: public_key};
  return serde_json::to_string(&key_pair).unwrap();
}

#[wasm_bindgen]
pub fn decrypt (pk: &str, data: &[u8]) -> Vec<u8> {
  let private_key: RSAPrivateKey = serde_json::from_str(pk).unwrap();
  return private_key.decrypt(PaddingScheme::PKCS1v15, data).expect("failed to decrypt");
}

#[wasm_bindgen]
pub fn encrypt (pk: &str, data: &[u8]) -> Vec<u8> {
  let mut rng = OsRng;
  let public_key: RSAPublicKey = serde_json::from_str(pk).unwrap();
  return public_key.encrypt(&mut rng, PaddingScheme::PKCS1v15, data).expect("failed to encrypt");
}
