use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(context: &str, s: &str) -> String {
  let r = String::from("hello ");
  let ret = "
    {
      'callback': {
        'method': 'POST',
        'hostname': 'api.sendgrid.com',
        'port': 443,
        'path': '/v3/mail/send',
        'headers': {
          'Content-Type': 'application/json',
          'authorization': 'Bearer AUTH_TOKEN'
        },
        'maxRedirects': 20
      },
      'personalizations': {
        [{
          'to':[{'email':'TO_EMAIL','name':''}],
          'subject':'SUBJECT'
        }],
        'from':{'email':'FROM_EMAIL','name':''}
      }
    }
  ";

  let ret = ret.replace("AUTH_TOKEN", "auth_token_123");
  let ret = ret.replace("TO_EMAIL", "alice@secondstate.io");
  let ret = ret.replace("SUBJECT", &(r + &s));
  let ret = ret.replace("FROM_EMAIL", "dev@developer.com");
  return ret;
}
