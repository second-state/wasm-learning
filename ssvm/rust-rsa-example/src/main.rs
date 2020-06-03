use clap::{Arg, App, SubCommand};
use rsa::{PublicKey, RSAPublicKey, RSAPrivateKey, PaddingScheme};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct RSAKeyPair {
  rsa_private_key: RSAPrivateKey,
  rsa_public_key: RSAPublicKey
}
pub fn generate_key_pair (bits: i32) -> String {
  let mut rng = OsRng;
  let private_key = RSAPrivateKey::new(&mut rng, bits as usize).expect("failed to generate a key");
  let public_key = private_key.to_public_key();
  let key_pair = RSAKeyPair {rsa_private_key: private_key, rsa_public_key: public_key};
  return serde_json::to_string(&key_pair).unwrap();
}
pub fn decrypt (pk: &str, data: &[u8]) -> Vec<u8> {
  let private_key: RSAPrivateKey = serde_json::from_str(pk).unwrap();
  return private_key.decrypt(PaddingScheme::PKCS1v15, data).expect("failed to decrypt");
}
pub fn encrypt (pk: &str, data: &[u8]) -> Vec<u8> {
  let mut rng = OsRng;
  let public_key: RSAPublicKey = serde_json::from_str(pk).unwrap();
  return public_key.encrypt(&mut rng, PaddingScheme::PKCS1v15, data).expect("failed to encrypt");
}
fn main() {
  let matches = App::new("RSA Example")
    .subcommand(SubCommand::with_name("generate_key_pair")
                .arg(Arg::with_name("bits")
                     .short("b")
                     .long("bits")
                     .value_name("BITS")
                     .index(1)
                     ))
    .subcommand(SubCommand::with_name("decrypt")
                .arg(Arg::with_name("private_key")
                     .short("k")
                     .long("key")
                     .value_name("PRIVATE_KEY_JSON")
                     .required(true)
                     .index(1)
                     )
                .arg(Arg::with_name("data")
                     .short("d")
                     .long("data")
                     .value_name("DATA_JSON")
                     .required(true)
                     .index(2)
                     ))
    .subcommand(SubCommand::with_name("encrypt")
                .arg(Arg::with_name("public_key")
                     .short("k")
                     .long("key")
                     .value_name("PUBLIC_KEY_JSON")
                     .required(true)
                     .index(1)
                     )
                .arg(Arg::with_name("data")
                     .short("d")
                     .long("data")
                     .value_name("DATA_JSON")
                     .required(true)
                     .index(2)
                     ))
    .get_matches();
  if let Some(matches) = matches.subcommand_matches("generate_key_pair") {
    let bits = matches
      .value_of("bits")
      .and_then(|x| x.parse::<i32>().ok())
      .unwrap_or(2048);
    println!("{}", generate_key_pair(bits));
  } else if let Some(matches) = matches.subcommand_matches("encrypt") {
    let pk = matches
      .value_of("public_key")
      .unwrap();
    let data = matches
      .value_of("data")
      .unwrap();
    println!("data:'{}'", data);
    let data_bytes = data.as_bytes();
    let result = encrypt(pk, &data_bytes);
    let result_json = serde_json::to_string(&result).unwrap();
    println!("encrypt:'{}'", result_json);
  } else if let Some(matches) = matches.subcommand_matches("decrypt") {
    let pk = matches
      .value_of("private_key")
      .unwrap();
    let data = matches
      .value_of("data")
      .and_then(|x| serde_json::from_str::<Vec<u8>>(x).ok())
      .expect("failed to decode json string");
    println!("data:'{}'", serde_json::to_string(&data).unwrap());
    let result = decrypt(pk, &data);
    let result_utf8 = String::from_utf8(result).expect("failed to decode utf8 bytes");
    println!("decrypt:'{}'", result_utf8);
  }
}
