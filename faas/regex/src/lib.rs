use wasm_bindgen::prelude::*;
use regex::Regex;
// use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub fn match_text (params: &str) -> String {
    let (regex, text) : (String, String) = serde_json::from_str(&params).unwrap();
    let mut vec : Vec<String> = Vec::new();
    for mat in Regex::new(&regex).unwrap().find_iter(&text) {
        // println!("{:?}", mat);
        vec.push(mat.as_str().to_string());
    }
    return serde_json::to_string(&vec).unwrap();
}
