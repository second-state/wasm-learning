use wasm_bindgen::prelude::*;
use std::env;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
    // Access arguments from std env
    let arguments: Vec<String> = env::args().collect();
    // Convert argument in position 1 to boolean
    let use_emoji_bool: bool = arguments[1].parse().unwrap();
    if use_emoji_bool {
        let r = String::from("👋 ");
        return r + &s;
    } else {
        let r = String::from("hello ");
        return r + &s;
    }
}
