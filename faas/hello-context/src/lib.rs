use wasm_bindgen::prelude::*;
use std::env;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let arguments: Vec<String> = env::args().collect();
  let use_emoji = arguments[1].parse().unwrap();
  if use_emoji {
    let r = String::from("👋 ");
    return r + &s;
  } else {
    let r = String::from("hello ");
    return r + &s;
  }
}
