use wasm_bindgen::prelude::*;
use core::mem;

#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  pub fn log(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  pub fn time(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  pub fn timeEnd(s: &str);
}

// Provide a function name that comforms to Rust's convention
pub fn time_end(s: &str) {
    timeEnd(s);
}
