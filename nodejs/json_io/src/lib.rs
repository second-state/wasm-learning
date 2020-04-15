use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
  x: f32, 
  y: f32
}

#[derive(Serialize, Deserialize, Debug)]
struct Line {
  points: Vec<Point>,
  valid: bool,
  length: f32,
  desc: String
}

#[wasm_bindgen]
pub fn circumference(radius: &str) -> String {
  let r: f32 = serde_json::from_str(radius).unwrap();
  let c = 2. * 3.14159 * r;
  return serde_json::to_string(&c).unwrap();
}

#[wasm_bindgen]
pub fn area(sides: &str) -> String {
  let s: (f32, f32) = serde_json::from_str(&sides).unwrap();
  let a = s.0 * s.1;
  return serde_json::to_string(&a).unwrap();
}

#[wasm_bindgen]
pub fn solve(params: &str) -> String {
  let ps: (f32, f32, f32) = serde_json::from_str(&params).unwrap();
  let discriminant: f32 = (ps.1 * ps.1) - (4. * ps.0 * ps.2);
  let mut solution: (f32, f32, bool) = (0., 0., false);
  if discriminant >= 0. {
    solution.0 = (((-1.) * ps.1) + discriminant.sqrt()) / (2. * ps.0);
    solution.1 = (((-1.) * ps.1) - discriminant.sqrt()) / (2. * ps.0);
    solution.2 = true;
  }
  return serde_json::to_string(&solution).unwrap();
}

#[wasm_bindgen]
pub fn draw(points: &str) -> String {
  let ps: (Point, Point, String) = serde_json::from_str(&points).unwrap();
  let length = ((ps.0.x - ps.1.x) * (ps.0.x - ps.1.x) + (ps.0.y - ps.1.y) * (ps.0.y - ps.1.y)).sqrt();
  
  let valid = if length == 0.0 { false } else { true };

  let line = Line { points: vec![ps.0, ps.1], valid: valid, length: length, desc: ps.2 };

  return serde_json::to_string(&line).unwrap();
}

