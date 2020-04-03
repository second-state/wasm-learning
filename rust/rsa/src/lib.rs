use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;

pub fn generate_private_key (bits: i32, passphrase: &str) -> Vec<u8> {
  let rsa = Rsa::generate(bits as u32).unwrap();
  return rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), passphrase.as_bytes()).unwrap();
}

pub fn get_public_key (private_key_pem: &[u8], passphrase: &str) -> Vec<u8> {
  let rsa = Rsa::private_key_from_pem_passphrase(private_key_pem, passphrase.as_bytes()).unwrap();
  return rsa.public_key_to_pem().unwrap();
}


pub fn public_encrypt (public_key_pem: &[u8], data: &[u8]) -> Vec<u8> {
  let rsa = Rsa::public_key_from_pem(public_key_pem).unwrap();
  let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
  let _ = rsa.public_encrypt(data, &mut buf, Padding::PKCS1).unwrap();
  return buf;
}

pub fn private_decrypt (private_key_pem: &[u8], passphrase: &str, data: &[u8]) -> Vec<u8> {
  let rsa = Rsa::private_key_from_pem_passphrase(private_key_pem, passphrase.as_bytes()).unwrap();
  let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
  let _ = rsa.private_decrypt(data, &mut buf, Padding::PKCS1).unwrap();
  return buf;
}
