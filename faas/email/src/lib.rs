use wasm_bindgen::prelude::*;
use std::env;
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Msg {
  to: String,
  subject: String,
  mime: String,
  body: String
}

#[wasm_bindgen]
pub fn send_email(s: &str) -> String {
  // Access arguments from std env
  let arguments: Vec<String> = env::args().collect();
  let from: &str = &arguments[1];

  let msg: Msg = serde_json::from_str(s).unwrap();
  let ret = json!(
    {
        "personalizations": [{
            "to": [{
                "email": &msg.to
            }]
        }],
        "from": {
            "email": from
        },
        "subject":&msg.subject,
        "content": [{
            "type": &msg.mime,
            "value": &msg.body
        }]
    });
  return ret.to_string();
}
