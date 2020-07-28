use wasm_bindgen::prelude::*;

// Solve the square of a given number
#[wasm_bindgen]
pub fn solve(params: &str) -> String {
  let ps: (f32) = serde_json::from_str(&params).unwrap();
  let discriminant: f32 = (ps.0);
  if discriminant >= 0. {
    solution.0 = (ps.0 * ps.0);
    return serde_json::to_string(&solution).unwrap();
  } else {
    return String::from("not real numbers");
  }
}
