use wasm_bindgen::prelude::*;
use serde_json;
use tera::{Context, Tera};

#[derive(Clone, Debug)]
pub struct Graph {
    pub name: String,
    pub size: usize,
    pub points: Vec<Point>,
    pub colour: String,
    pub max_x: f64,
    pub max_y: f64
}

#[derive(Clone, Debug, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Graph {
  pub fn new(name: String, colour: String) -> Self {
      Graph {
          name,
          size: 0,
          points: Vec::new(),
          colour,
          max_x : 0.,
          max_y : 0.,
      }
  }

  pub fn add_point(&mut self, x: f64, y: f64) {
      self.points.push(Point { x, y });
  }

  pub fn draw_svg(&self, width: i32, height: i32, padding: i32, path: String) -> String {

    let mut context = Context::new();

    //let min_x = graph.points.get(0).map(|val| val.x).unwrap_or(0.0);
    /*let max_x = self
      .points
      .iter()
      .map(|point| point.x)
      .fold(0. / 0., f64::max);

    //let min_y = graph.points.iter().map(|val| val.y).fold(0. / 0., f64::min);
    let max_y = self
      .points
      .iter()
      .map(|point| point.y)
      .fold(0. / 0., f64::max);
      //hardset the padding around the graph
    */
    // let c_str = CString::new(file).unwrap();
    // let filename: *const c_char = c_str.as_ptr() as *const c_char;
    // const filename: &str = file.clone();

    //ensure the viewbox is as per input
  
    context.insert("name", &self.name);
    context.insert("width", &width);
    context.insert("height", &height);
    context.insert("padding", &padding);
    context.insert("path", &path);
    context.insert("max_x", &self.max_x);
    context.insert("max_y", &self.max_y);
    context.insert("colour", &self.colour);
    context.insert("lines", &5);
  
    Tera::one_off(include_str!("graph.svg"), &context, true).expect("Could not draw graph")
    
  }
}


pub fn generate_graph(xs: Vec<f64>, ys: Vec<f64>, title : &str) -> Graph {
  let mut graph = Graph::new(title.into(), "#8ff0a4".into());
  graph.size = xs.len();
  for i in 0..graph.size {
    graph.add_point(xs[i], ys[i]);
  }
  return graph;
} 

#[wasm_bindgen]
pub fn get_svg(xstr: &str, ystr: &str, width: i32, height: i32, padding: i32, title: &str) -> String {
  let xs: Vec<f64> = serde_json::from_str(&xstr).unwrap();
  let ys: Vec<f64> = serde_json::from_str(&ystr).unwrap();
  let mut graph = generate_graph(xs, ys, title);
  let width = width - padding * 2;
  let height = height - padding * 2;
  //let min_x = graph.points.get(0).map(|val| val.x).unwrap_or(0.0);
  graph.max_x = graph
    .points
    .iter()
    .map(|point| point.x)
    .fold(0. / 0., f64::max);
  
  //let min_y = graph.points.iter().map(|val| val.y).fold(0. / 0., f64::min);
  graph.max_y = graph
    .points
    .iter()
    .map(|point| point.y)
    .fold(0. / 0., f64::max);

  let path = graph
              .points
              .iter()
              .map(|val| Point {
                  x: (val.x / graph.max_x * width as f64) + padding as f64,
                  y: (val.y / graph.max_y * (height as f64 * -1.0)) + (padding + height) as f64,
              })
              .enumerate()
              .map(|(i, point)| {
                  if i == 0 {
                      format!("M {} {}", point.x, point.y)
                  } else {
                      format!("L {} {}", point.x, point.y)
                  }
              })
              .collect::<Vec<String>>().join(" ");

  let out = graph.draw_svg(width, height, padding, path);
  // println!("{}", out);
  return out;
}
