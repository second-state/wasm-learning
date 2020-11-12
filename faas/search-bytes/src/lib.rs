use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn search_bytes(haystack: &[u8], needle: &[u8]) -> String {
    match haystack.windows(needle.len()).position(|window| window == needle){
        Some(_) => "Present".to_string(),
        None => "Absent".to_string()
    }
}