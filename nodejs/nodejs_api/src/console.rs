use wasm_bindgen::prelude::*;
use core::mem;

#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  fn time(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  fn timeEnd(s: &str);
}


pub fn console_log(msg: &str) {
    assert_eq!(mem::size_of::<usize>(), 4);
    log(msg);
}

pub fn console_time(label: &str) {
    assert_eq!(mem::size_of::<usize>(), 4);
    time(label);
}

pub fn console_time_end(label: &str) {
    assert_eq!(mem::size_of::<usize>(), 4);
    timeEnd(label);
}
