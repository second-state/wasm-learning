use wasm_bindgen::prelude::*;
use serde_json::json;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
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
        "subject": &s,
        "content": [{
            "type": "text/plain",
            "value": &("This is a message from Second State: ".to_owned() + s)
        }]
    });
  return ret.to_string();
}
