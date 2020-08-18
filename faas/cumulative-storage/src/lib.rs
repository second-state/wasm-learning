use std::env;
use wasm_bindgen::prelude::*;
use rust_storage_interface_library::ssvm_storage;
use ssvm_wasi_helper::ssvm_wasi_helper::_initialize;

// This example demonstrates cumulative storage

// First we initialize the storage with a value i.e. 1
#[wasm_bindgen]
pub fn init(_string_to_store: String) -> String {
    _initialize();
    let storage_key: String = env::var("storage_key").unwrap();
    ssvm_storage::store::update(&storage_key, _string_to_store);
    storage_key
}

// Then we can demonstrate how we can increment the value by one each time the increment function
// is called
#[wasm_bindgen]
pub fn increment() -> String {
    _initialize();
    let storage_key: String = env::var("storage_key").unwrap();
    let retrieved_string: String = ssvm_storage::load::load_as_string(&storage_key);
    let as_int: i32 = retrieved_string.parse().unwrap();
    let answer: i32 = as_int + 1;
    ssvm_storage::store::update(&storage_key, answer.to_string());
    answer.to_string()
} 
