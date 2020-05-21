use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serialize_deserialize_u8_i32::s_d_u8_i32;
use rust_storage_interface_library::ssvm_storage;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
struct TestStruct {
    a_vec: Vec<u8>,
    a_i32: i32,
    a_u8: u8,
    a_bool: bool,
}

#[wasm_bindgen]
pub fn store_primitive_types (data: &str) -> String {
  let to_store: (bool, char, i64, f64) = serde_json::from_str(data).unwrap();
  let key_0 = ssvm_storage::store::store(to_store.0);
  let key_1 = ssvm_storage::store::store(to_store.1);
  let key_2 = ssvm_storage::store::store(to_store.2);
  let key_3 = ssvm_storage::store::store(to_store.3);
  return serde_json::to_string(&(key_0, key_1, key_2, key_3)).unwrap();
}

#[wasm_bindgen]
pub fn load_primitive_types (ks: &str) -> String {
  let keys: (i32, i32, i32, i32) = serde_json::from_str(ks).unwrap();
  let d_0 = ssvm_storage::load::load_as_boolean(keys.0);
  let d_1 = ssvm_storage::load::load_as_char(keys.1);
  let d_2 = ssvm_storage::load::load_as_i64(keys.2);
  let d_3 = ssvm_storage::load::load_as_f64(keys.3);
  return serde_json::to_string(&(d_0, d_1, d_2, d_3)).unwrap();
}

#[wasm_bindgen]
pub fn store_string (data: &str) -> i32 {
  let key = ssvm_storage::store::store(data);
  return key;
}

#[wasm_bindgen]
pub fn load_string (key: i32) -> String {
  let data = ssvm_storage::load::load_as_string(key);
  return data;
}

#[wasm_bindgen]
pub fn store_struct (data: &str) -> i32 {
  let to_store: TestStruct = serde_json::from_str(data).unwrap();
  let key = ssvm_storage::store::store(to_store);
  return key;
}

#[wasm_bindgen]
pub fn load_struct (key: i32) -> String {
  let struct_skeleton = TestStruct::default();
  let d: TestStruct = ssvm_storage::load::load_as_struct(struct_skeleton, key);
  return serde_json::to_string(&d).unwrap();
}

#[wasm_bindgen]
pub fn store_bytes (data: &[u8]) -> i32 {
  let bytes_in_i32: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32((*data).to_vec());
  let key = ssvm_storage::store::store(bytes_in_i32);
  return key;
}

#[wasm_bindgen]
pub fn load_bytes (key: i32) -> Vec<u8> {
  let bytes_in_i32: Vec<i32> = ssvm_storage::load::load_as_i32_vector(key);
  return s_d_u8_i32::deserialize_i32_to_u8(bytes_in_i32);
}
