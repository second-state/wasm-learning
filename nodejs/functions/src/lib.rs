extern crate num_integer;

use wasm_bindgen::prelude::*;
use num_integer::lcm;
use sha3::{Digest, Sha3_256, Keccak256};

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let r = String::from("hello ");
  return r + s;
}

#[wasm_bindgen]
pub fn obfusticate(s: String) -> String {
  return rot13(&s);
}

#[wasm_bindgen]
pub fn lowest_common_denominator(a: i32, b: i32) -> i32 {
  let r = lcm(a, b);
  return r;
}

#[wasm_bindgen]
pub fn sha3_digest(v: Vec<u8>) -> Vec<u8> {
  return Sha3_256::digest(&v).as_slice().to_vec();
}

#[wasm_bindgen]
pub fn keccak_digest(s: &[u8]) -> Vec<u8> {
  return Keccak256::digest(s).as_slice().to_vec();
}

pub fn rot13(text: &str) -> String {
    text.chars().map(|c| {
        match c {
            'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + 13) as char,
            'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - 13) as char,
            _ => c
        }
    }).collect()
}
