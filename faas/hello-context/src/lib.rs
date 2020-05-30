use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(context: &str, s: &str) -> String {
  if context == "emoji" {
    let r = String::from("👋 ");
    return r + &s;
  } else {
    let r = String::from("hello ");
    return r + &s;
  }
}
