use wasm_bindgen::prelude::*;

#[wasm_bindgen(module="/node/sqlite-helper.js")]
extern "C" {
    pub fn create(path: &str);
    pub fn update(path: &str, sql: &str);
    pub fn exec(path: &str, sql: &str);
    pub fn query(path: &str, sql: &str) -> String;
}
