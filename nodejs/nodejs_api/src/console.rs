use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  pub fn log(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  pub fn error(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  pub fn time(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  pub fn timeEnd(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  pub fn timeLog(s: &str, v: &str);
}

// Provide function names that comform to Rust's convention
pub fn time_end(s: &str) {
    timeEnd(s);
}

pub fn time_log(s: &str, v: &str) {
    timeLog(s, v);
}
