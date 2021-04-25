use wasm_bindgen::prelude::*;
use serde_json::{Value};

#[wasm_bindgen]
pub fn say(slack_object_as_string: &str) -> String {
  let hello_prefix = String::from("Hello ");
  let json_object_from_slack: Value = serde_json::from_str(&slack_object_as_string).unwrap();
  let hello_suffix: String = json_object_from_slack["text"].to_string();
  return hello_prefix + &hello_suffix;
}
