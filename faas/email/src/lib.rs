use wasm_bindgen::prelude::*;
use std::str;
use serde_json::json;
use serde::{Serialize, Deserialize};
use rust_process_interface_library::Command;

#[derive(Serialize, Deserialize, Debug)]
struct Msg {
  from: String,
  token: String,
  to: String,
  subject: String,
  mime: String,
  body: String
}

#[wasm_bindgen]
pub fn send_email(s: &str) -> String {
  let msg: Msg = serde_json::from_str(s).unwrap();
  let payload = json!(
    {
        "personalizations": [{
            "to": [{
                "email": &msg.to
            }]
        }],
        "from": {
            "email": &msg.from
        },
        "subject":&msg.subject,
        "content": [{
            "type": &msg.mime,
            "value": &msg.body
        }]
    });
  let auth_header: String = "{\"Content-Type\": \"application/json\",\"authorization\": \"Bearer ".to_owned() + &msg.token + "\"}";

  let mut cmd = Command::new("http_proxy");
  cmd.arg("post")
      .arg("https://api.sendgrid.com/v3/mail/send")
      .arg(auth_header);
  cmd.stdin_u8vec(payload.to_string().as_bytes());

  let out = cmd.output();
  if out.status != 0 {
      println!("Code: {}", out.status);
      println!("STDERR: {}", str::from_utf8(&out.stderr).unwrap());
      println!("STDOUT: {}", str::from_utf8(&out.stdout).unwrap());
      return out.status.to_string();
  }

  return str::from_utf8(&out.stdout).unwrap().to_string();
}
