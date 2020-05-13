use recrypt::prelude::*;
use recrypt::api::*;
use wasm_bindgen::prelude::*;
// use nodejs_helper;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Identity {
    public_key_x: Vec<u8>,
    public_key_y: Vec<u8>,
    private_key: Vec<u8>,
    signing_key_pair: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
struct SSTransformKey {
    ephemeral_public_key_x: Vec<u8>,
    ephemeral_public_key_y: Vec<u8>,
    to_public_key_x: Vec<u8>,
    to_public_key_y: Vec<u8>,
    encrypted_temp_key: Vec<u8>,
    hashed_temp_key: Vec<u8>,
    public_signing_key: Vec<u8>,
    signature: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
struct SSEncryptedValue {
    ephemeral_public_key_x: Vec<u8>,
    ephemeral_public_key_y: Vec<u8>,
    encrypted_message: Vec<u8>,
    auth_hash: Vec<u8>,
    public_signing_key: Vec<u8>,
    signature: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
struct SSTransformedValue {
    ephemeral_public_key_x: Vec<u8>,
    ephemeral_public_key_y: Vec<u8>,
    encrypted_message: Vec<u8>,
    auth_hash: Vec<u8>,
    public_signing_key: Vec<u8>,
    signature: Vec<u8>,
    transform_public_key_x: Vec<u8>,
    transform_public_key_y: Vec<u8>,
    transform_encrypted_temp_key: Vec<u8>,
    transform_random_transform_public_key_x: Vec<u8>,
    transform_random_transform_public_key_y: Vec<u8>,
    transform_encrypted_random_transform_temp_key: Vec<u8>
}

#[wasm_bindgen]
pub fn create_identity () -> String {
    let recrypt = Recrypt::new();
    let (priv_key, pub_key) = recrypt.generate_key_pair().unwrap();
    let signing_keypair= recrypt.generate_ed25519_key_pair();

    let pub_key_bytes = pub_key.bytes_x_y();
    let result = Identity {
        public_key_x: pub_key_bytes.0.to_vec(),
        public_key_y: pub_key_bytes.1.to_vec(),
        private_key: priv_key.bytes().to_vec(),
        signing_key_pair: signing_keypair.bytes().to_vec()
    };

    return serde_json::to_string(&result).unwrap();
}

#[wasm_bindgen]
pub fn create_plain_text () -> String {
    let recrypt = Recrypt::new();
    let pt = recrypt.gen_plaintext();
    return serde_json::to_string(&pt.bytes().to_vec()).unwrap();
}

#[wasm_bindgen]
pub fn get_symmetric_key (pt: &[u8]) -> Vec<u8> {
    let plain = Plaintext::new_from_slice(pt).unwrap();

    let recrypt = Recrypt::new();
    let sk = recrypt.derive_symmetric_key(&plain);
    return sk.bytes().to_vec();
}

// NOTE: This should be done on the client side. This is just a server side demo
// Input: plain_text, pub_key_x, pub_key_y, signing_key_pair
// Return: SSEncryptedValue
#[wasm_bindgen]
pub fn encrypt (ps: &str) -> String {
    // nodejs_helper::console::log(ps);
    let params: (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) = serde_json::from_str(ps).unwrap();
    let plain_text: Plaintext = Plaintext::new_from_slice(&params.0).unwrap();
    let public_key: PublicKey = PublicKey::new_from_slice((&params.1, &params.2)).unwrap();
    let signing_key_pair: SigningKeypair = SigningKeypair::from_byte_slice(&params.3).unwrap();

    let recrypt = Recrypt::new();
    let encrypted_val = recrypt.encrypt(&plain_text, &public_key, &signing_key_pair).unwrap();
    match encrypted_val {
        EncryptedValue::EncryptedOnceValue {ephemeral_public_key, encrypted_message, auth_hash, public_signing_key, signature} => {
            let result = SSEncryptedValue {
                ephemeral_public_key_x: ephemeral_public_key.bytes_x_y().0.to_vec(),
                ephemeral_public_key_y: ephemeral_public_key.bytes_x_y().1.to_vec(),
                encrypted_message: encrypted_message.bytes().to_vec(),
                auth_hash: auth_hash.bytes().to_vec(),
                public_signing_key: public_signing_key.bytes().to_vec(),
                signature: signature.bytes().to_vec()
            };
            return serde_json::to_string(&result).unwrap();
        },
        EncryptedValue::TransformedValue {..} => {
            return String::from("");
        }
    }
}

// NOTE: This should be done on the client side. This is just a server side demo
// Input: org_private_key, target_public_key_x, target_public_key_y, org_signing_key_pair [Identity_1 and Identity_2]
// Return: SSTransformKey
#[wasm_bindgen]
pub fn create_transform_key (ps: &str) -> String {
    let params: (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) = serde_json::from_str(ps).unwrap();
    let org_private_key: PrivateKey = PrivateKey::new_from_slice(&params.0).unwrap();
    let target_public_key: PublicKey = PublicKey::new_from_slice((&params.1, &params.2)).unwrap();
    let org_signing_key_pair: SigningKeypair = SigningKeypair::from_byte_slice(&params.3).unwrap();

    let recrypt = Recrypt::new();
    let transform_key = recrypt.generate_transform_key(&org_private_key, &target_public_key, &org_signing_key_pair).unwrap();
    let result = SSTransformKey {
        ephemeral_public_key_x: transform_key.ephemeral_public_key().bytes_x_y().0.to_vec(),
        ephemeral_public_key_y: transform_key.ephemeral_public_key().bytes_x_y().1.to_vec(),
        to_public_key_x: transform_key.to_public_key().bytes_x_y().0.to_vec(),
        to_public_key_y: transform_key.to_public_key().bytes_x_y().1.to_vec(),
        encrypted_temp_key: transform_key.encrypted_temp_key().bytes().to_vec(),
        hashed_temp_key: transform_key.hashed_temp_key().bytes().to_vec(),
        public_signing_key: transform_key.public_signing_key().bytes().to_vec(),
        signature: transform_key.signature().bytes().to_vec()
    };
    return serde_json::to_string(&result).unwrap();
}

// Input: SSEncryptedValue, SSTransformKey, signing_key_pair
// Return: SSTransformedValue
#[wasm_bindgen]
pub fn transform (ps: &str) -> String {
    let (ev, tk, skp): (SSEncryptedValue, SSTransformKey, Vec<u8>) = serde_json::from_str(ps).unwrap();
    let encrypted_val = EncryptedValue::EncryptedOnceValue {
        ephemeral_public_key: PublicKey::new_from_slice((&ev.ephemeral_public_key_x, &ev.ephemeral_public_key_y)).unwrap(),
        encrypted_message: EncryptedMessage::new_from_slice(&ev.encrypted_message).unwrap(),
        auth_hash: AuthHash::new_from_slice(&ev.auth_hash).unwrap(),
        public_signing_key: PublicSigningKey::new_from_slice(&ev.public_signing_key).unwrap(),
        signature: Ed25519Signature::new_from_slice(&ev.signature).unwrap()
    };

    let transform_key = TransformKey::new (
        PublicKey::new_from_slice((&tk.ephemeral_public_key_x, &tk.ephemeral_public_key_y)).unwrap(),
        PublicKey::new_from_slice((&tk.to_public_key_x, &tk.to_public_key_y)).unwrap(),
        EncryptedTempKey::new_from_slice(&tk.encrypted_temp_key).unwrap(),
        HashedValue::new_from_slice(&tk.hashed_temp_key).unwrap(),
        PublicSigningKey::new_from_slice(&tk.public_signing_key).unwrap(),
        Ed25519Signature::new_from_slice(&tk.signature).unwrap()
    );

    let signing_key_pair = SigningKeypair::from_byte_slice(&skp).unwrap();

    let recrypt = Recrypt::new();
    let transformed_val = recrypt.transform(encrypted_val, transform_key, &signing_key_pair).unwrap();
    match transformed_val {
        EncryptedValue::TransformedValue {ephemeral_public_key, encrypted_message, auth_hash, transform_blocks, public_signing_key, signature} => {
            let result = SSTransformedValue {
                ephemeral_public_key_x: ephemeral_public_key.bytes_x_y().0.to_vec(),
                ephemeral_public_key_y: ephemeral_public_key.bytes_x_y().1.to_vec(),
                encrypted_message: encrypted_message.bytes().to_vec(),
                auth_hash: auth_hash.bytes().to_vec(),
                public_signing_key: public_signing_key.bytes().to_vec(),
                signature: signature.bytes().to_vec(),
                transform_public_key_x: transform_blocks.first().public_key().bytes_x_y().0.to_vec(),
                transform_public_key_y: transform_blocks.first().public_key().bytes_x_y().1.to_vec(),
                transform_encrypted_temp_key: transform_blocks.first().encrypted_temp_key().bytes().to_vec(),
                transform_random_transform_public_key_x: transform_blocks.first().random_transform_public_key().bytes_x_y().0.to_vec(),
                transform_random_transform_public_key_y: transform_blocks.first().random_transform_public_key().bytes_x_y().1.to_vec(),
                transform_encrypted_random_transform_temp_key: transform_blocks.first().encrypted_random_transform_temp_key().bytes().to_vec()
            };
            return serde_json::to_string(&result).unwrap();
        },
        EncryptedValue::EncryptedOnceValue {..} => {
            return String::from("");
        }
    }
}

// NOTE: This should be done on the client side. This is just a server side demo
// Input: SSTransformedValue, target_private_key [Identity]
// Return: bytes for plain text
#[wasm_bindgen]
pub fn decrypt (v: &str) -> String {
    let (tv, pk): (SSTransformedValue, Vec<u8>) = serde_json::from_str(v).unwrap();

    let tb = TransformBlock::new (
        &PublicKey::new_from_slice((&tv.transform_public_key_x, &tv.transform_public_key_y)).unwrap(),
        &EncryptedTempKey::new_from_slice(&tv.transform_encrypted_temp_key).unwrap(),
        &PublicKey::new_from_slice((&tv.transform_random_transform_public_key_x, &tv.transform_random_transform_public_key_y)).unwrap(),
        &EncryptedTempKey::new_from_slice(&tv.transform_encrypted_random_transform_temp_key).unwrap()
    ).unwrap();
    let transformed_val = EncryptedValue::TransformedValue {
        ephemeral_public_key: PublicKey::new_from_slice((&tv.ephemeral_public_key_x, &tv.ephemeral_public_key_y)).unwrap(),
        encrypted_message: EncryptedMessage::new_from_slice(&tv.encrypted_message).unwrap(),
        auth_hash: AuthHash::new_from_slice(&tv.auth_hash).unwrap(),
        public_signing_key: PublicSigningKey::new_from_slice(&tv.public_signing_key).unwrap(),
        transform_blocks: recrypt::nonemptyvec::NonEmptyVec::new_first(tb),
        signature: Ed25519Signature::new_from_slice(&tv.signature).unwrap()
    };
    let target_private_key = PrivateKey::new_from_slice(&pk).unwrap();

    let recrypt = Recrypt::new();
    let decrypted_val = recrypt.decrypt(transformed_val, &target_private_key).unwrap();
    return serde_json::to_string(&decrypted_val.bytes().to_vec()).unwrap();
    // return decrypted_val.bytes().to_vec();
}
