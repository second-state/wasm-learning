use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let r = String::from("hello ");
  let ret = "
    {
      'personalizations': {
        [{
          'to':[{'email':'TO_EMAIL','name':''}],
          'subject':'SUBJECT'
        }],
        'from':{'email':'FROM_EMAIL','name':''}
      }
    }
  ";
  
  let ret = ret.replace("TO_EMAIL", "juntao_yuan@yahoo.com");
  let ret = ret.replace("SUBJECT", &(r + &s));
  let ret = ret.replace("FROM_EMAIL", "michael@secondstate.io");
  return ret;
}
