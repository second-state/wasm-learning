use std::env;
use serde_json;
use serde_json::{Value};
use wasm_bindgen::prelude::*;
use rust_storage_interface_library::ssvm_storage;
// This example lets you create and manage new keys for every new store operation
#[wasm_bindgen]
pub fn store_a_string(_string_to_store: String) -> String {
    let storage_key: String = ssvm_storage::store::store(_string_to_store);
    storage_key
}
// This example requires the keys that you are managing to be passed in so that data can be accessed
#[wasm_bindgen]
pub fn load_a_string(_storage_key: String) -> String {
    let retrieved_string: String = ssvm_storage::load::load_as_string(&_storage_key);
    retrieved_string
}
// This example manages a storage key for you. It is available via std env inside Rust so there is no need to pass strings from calling code via RPC
#[wasm_bindgen]
pub fn store_a_string_via_std_env(_string_to_store: String) {
    // Get the storage key from std env
    let json_as_object: Value = serde_json::from_str(env::env()).unwrap();
    let storage_key: String = json_as_object["storage_key"].to_string(); 
    // Store the _string_to_store 
    ssvm_storage::store::update(&storage_key, _string_to_store);
}
// This example shows how a string can be loaded from storage using a key which is available via std env so there is no need to pass strings from calling code
#[wasm_bindgen]
pub fn load_a_string_via_std_env() -> String {
    // Get the storage key from std env
    let json_as_object: Value = serde_json::from_str(env::env()).unwrap();
    let storage_key: String = json_as_object["storage_key"].to_string();
    // Use that key to load the string from permanent storage
    let retrieved_string: String = ssvm_storage::load::load_as_string(&storage_key);
    retrieved_string
}
