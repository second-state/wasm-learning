use wasm_bindgen::prelude::*;

#[wasm_bindgen(module="/node/request-helper.js")]
extern "C" {
    pub fn request(method: &str, url: &str) -> String;
    pub fn fetch(url: &str) -> Vec<u8>;
    pub fn fetch_as_string(url: &str) -> String;
    pub fn request_with_options(method: &str, url: &str, options: &str) -> String;
}
