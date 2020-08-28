use wasm_bindgen::prelude::*;
use std::env;

#[wasm_bindgen]
pub fn encode (data: &[u8]) -> String {
    // First arg is the context var
    let arguments: Vec<String> = env::args().collect();
    let mode: &str = &arguments[1];
    // println!("data size: {}", data.to_vec().len());
    // println!("encoded: {}", base64::encode_config(data, base64::STANDARD));

    if mode == "url_safe" {
        return base64::encode_config(data, base64::URL_SAFE);
    } else {
        return base64::encode_config(data, base64::STANDARD);
    }
}

#[wasm_bindgen]
pub fn decode (text: &str) -> Vec<u8> {
    // First arg is the context var
    let arguments: Vec<String> = env::args().collect();
    let mode: &str = &arguments[1];
    // println!("encoded: {}", text);
    // println!("data size: {}", base64::decode_config(text, base64::STANDARD).unwrap().len());

    if mode == "url_safe" {
        return base64::decode_config(text, base64::URL_SAFE).unwrap();
    } else {
        return base64::decode_config(text, base64::STANDARD).unwrap();
    }
}
