// Recrypt
use recrypt::prelude::*;

// Serde
use serde::{Deserialize, Serialize};
use serde_json;

// Bindgen
use wasm_bindgen::prelude::*;

#[no_mangle]
#[wasm_bindgen]
pub fn generate_key_pair() -> String {
    let mut recrypt = Recrypt::new();
    let (priv_key, pub_key) = recrypt.generate_key_pair().unwrap();
    serde_json::to_string(&priv_key).unwrap();
}
