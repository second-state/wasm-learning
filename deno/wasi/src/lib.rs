use wasm_bindgen::prelude::*;
use rand::prelude::*;
use std::env;

#[wasm_bindgen]
pub fn get_random_i32() -> i32 {
  let x: i32 = random();
  return x;
}

#[wasm_bindgen]
pub fn get_random_bytes() -> Vec<u8> {
  let mut rng = thread_rng();
  let mut arr = [0u8; 128];
  rng.fill(&mut arr[..]);
  return arr.to_vec();
  /*
  let mut vec: Vec<u8> = vec![0; 128];
  getrandom::getrandom(&mut vec).unwrap();
  return vec;
   */
}

#[wasm_bindgen]
pub fn echo(content: &str) -> String {
  println!("Printed from Deno wasi: {}", content);
  return content.to_string();
}

#[wasm_bindgen]
pub fn print_args() -> i32 {
  println!("The args are as follows.");
  for argument in env::args() {
    println!("{}", argument);
  }
  return 0;
}
