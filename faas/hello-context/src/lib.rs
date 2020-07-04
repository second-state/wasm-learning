use wasm_bindgen::prelude::*;
use std::env;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let emoji = env::var("EMOJI").unwrap_or_default(false);
  if emoji {
    let r = String::from("👋 ");
    return r + &s;
  } else {
    let r = String::from("hello ");
    return r + &s;
  }
}
