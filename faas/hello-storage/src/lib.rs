use wasm_bindgen::prelude::*;
use rust_storage_interface_library::ssvm_storage;

#[wasm_bindgen]
pub fn store_a_string(_string_to_store: String) -> String {
    let storage_key: String = ssvm_storage::store::store(_string_to_store);
    storage_key
}

#[wasm_bindgen]
pub fn load_a_string(_storage_key: String) -> String {
    let retrieved_string: String = ssvm_storage::load::load_as_string(&_storage_key);
    retrieved_string
}