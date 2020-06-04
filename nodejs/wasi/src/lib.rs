use wasm_bindgen::prelude::*;
use rand::prelude::*;

#[wasm_bindgen]
pub fn get_random_i32() -> i32 {
  let x: i32 = random();
  return x;
}

#[wasm_bindgen]
pub fn get_random_bytes() -> Vec<u8> {
  let mut vec: Vec<u8> = vec![0; 128];
  getrandom::getrandom(&mut vec).unwrap();
  return vec;
}

#[wasm_bindgen]
pub fn print_random_i32() -> i32 {
  let x: i32 = random();
  println!("Printed from wasm: A new random number is {}", x);
  return x;
}
