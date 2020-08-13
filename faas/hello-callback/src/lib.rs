use wasm_bindgen::prelude::*;
use serde_json::json;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let r = String::from("hello ");
  let ret = json!(
    {
        "personalizations": [{
            "to": [{
                "email": "michael@michaelyuan.com"
            }]
        }],
        "from": {
            "email": "michael@secondstate.io"
        },
        "subject":&(r + &s),
        "content": [{
            "type": "text/plain",
            "value": "This is a message from Joey and SSVM"
        }]
    });
  return ret.to_string();
}
