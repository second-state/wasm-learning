use tera::{Context, Tera};
use std::io::{self, Read};
use std::str::FromStr;
use std::iter::Iterator;
use std::string::String;
use serde::Deserialize;

extern crate rusty_machine as rm;
use rm::linalg::Matrix;
use rusty_machine::learning::pca::PCA;
use rm::learning::UnSupModel;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading from STDIN");
    let obj: FaasInput = serde_json::from_str(&buffer).unwrap();
    // println!("{}", obj.body);
    let csv_str = &(obj.body);

    let data: Vec<f64> = read_data(csv_str);
    let inputs = Matrix::new(data.len()/2, 2, data);
    let mut pca = PCA::default();
    pca.train(&inputs).unwrap();

    let graph = prepare_graph (csv_str, 800, 400, 20, "Principal Components Analysis");
    println!("{}", graph.pca(pca));
}

#[derive(Deserialize, Debug)]
struct FaasInput {
    body: String
}

#[derive(Clone, Debug)]
pub struct Graph {
    pub name: String,
    pub size: usize,
    pub points: Vec<Point>,
    pub colour: String,
    pub x_range: f64,
    pub y_range: f64,
    pub x_min: f64,
    pub y_min: f64,
    pub width: usize,
    pub height: usize,
    pub padding: usize,
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
          x_range : 0.,
          y_range : 0.,
          x_min : 0.,
          y_min : 0.,
          width: 0,
          height: 0,
          padding: 0,
      }
  }

  pub fn add_point(&mut self, x: f64, y: f64) {
      self.points.push(Point { x, y });
  }

  pub fn graph_map(&self, points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    points 
      .iter()
      .map(|val| ((val.0-self.x_min) / self.x_range * self.width as f64 + self.padding as f64, 
           (val.1-self.y_min) / self.y_range * (self.height as f64 * -1.0) + (self.padding + self.height) as f64)).collect()
  }

  pub fn pca(&self, pca: PCA) -> String {
    let mut context = self.create_svg_context();
    // let pca: PCA = serde_json::from_str(model).unwrap();

    let graph_center: (f64, f64) = (self.x_min + self.x_range/2.0, self.y_min + self.y_range/2.0);

    let eig_mat = pca.components().unwrap();
    let eig_vecs: Vec<f64> = eig_mat.data().to_vec();
    let mut eig_vecs_coefs: Vec<(f64, f64)> = Vec::new();
    for i in 0..(eig_vecs.len()/2) {
      eig_vecs_coefs.push((graph_center.1 - eig_vecs[2*i+1]/eig_vecs[2*i]*graph_center.0, eig_vecs[2*i+1]/eig_vecs[2*i]));
    }

    let mut eig_vecs_points : Vec<(f64, f64)> = Vec::new();
    for i in 0..eig_vecs_coefs.len() {
      if (eig_vecs_coefs[i].1 < 1.0) && (eig_vecs_coefs[i].1 > -1.0) {
        eig_vecs_points.push((self.x_min, eig_vecs_coefs[i].0 + eig_vecs_coefs[i].1 * self.x_min));
        eig_vecs_points.push((self.x_min + self.x_range, eig_vecs_coefs[i].0 + eig_vecs_coefs[i].1 * (self.x_min + self.x_range)));
      } else {
        eig_vecs_points.push(((self.y_min-eig_vecs_coefs[i].0) / eig_vecs_coefs[i].1, self.y_min));
        eig_vecs_points.push(((self.y_min+self.y_range-eig_vecs_coefs[i].0) / eig_vecs_coefs[i].1, self.y_min+self.y_range));
      }
    }
    eig_vecs_points = self.graph_map(eig_vecs_points);

    context.insert("eig_vector1_point1", &(eig_vecs_points[0]));
    context.insert("eig_vector1_point2", &(eig_vecs_points[1]));
    context.insert("eig_vector2_point1", &(eig_vecs_points[2]));
    context.insert("eig_vector2_point2", &(eig_vecs_points[3]));

    Tera::one_off(include_str!("pca.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn create_svg_context(&self) -> Context {
    let mut context = Context::new();
    let path: Vec<(f64, f64)> = self
                              .points
                              .iter()
                              .map(|val|
                                  //x: (val.x / graph.max_x * width as f64) + padding as f64,
                                  //y: (val.y / graph.max_y * (height as f64 * -1.0)) + (padding + height) as f64,
                                  (((val.x-self.x_min) / self.x_range * self.width as f64) + self.padding as f64,
                                   ((val.y-self.y_min) / self.y_range * (self.height as f64 * -1.0)) + (self.padding + self.height) as f64)
                                    ).collect();
    
    context.insert("name", &self.name);
    context.insert("width", &self.width);
    context.insert("height", &self.height);
    context.insert("padding", &self.padding);
    context.insert("path", &path);
    context.insert("x_range", &self.x_range);
    context.insert("y_range", &self.y_range);
    context.insert("x_min", &self.x_min);
    context.insert("y_min", &self.y_min);
    context.insert("colour", &self.colour);
    context.insert("lines", &10);

    return context;
  }
}

pub fn prepare_graph (csv_str: &str, width: usize, height: usize, padding: usize, title: &str) -> Graph {
  let data: Vec<f64> = read_data(csv_str);
  let mut xs: Vec<f64> = Vec::new();
  let mut ys: Vec<f64> = Vec::new();
  let mut tuples: Vec<(f64, f64)> = Vec::new();

  for i in 0..data.len() {
    if (i % 2) == 1 {
      tuples.push((data[i-1], data[i]));
    }
  }

  for i in 0..tuples.len() {
    xs.push(tuples[i].0);
    ys.push(tuples[i].1);
  }

  let width = width - padding * 2;
  let height = height - padding * 2;

  let mut graph = generate_graph(xs, ys, title, width, height, padding);

  let x_max = graph.points.iter().map(|point| point.x).fold(0. / 0., f64::max);
  let x_min = graph.points.iter().map(|point| point.x).fold(0. / 0., f64::min);
  let y_max = graph.points.iter().map(|point| point.y).fold(0. / 0., f64::max);
  let y_min = graph.points.iter().map(|point| point.y).fold(0. / 0., f64::min);

  graph.x_min = (x_min-1.0).round();
  graph.y_min = (y_min-1.0).round();

  graph.x_range = (x_max+1.0).round() - graph.x_min;
  graph.y_range = (y_max+1.0).round() - graph.y_min;

  return graph;
}

pub fn generate_graph(xs: Vec<f64>, ys: Vec<f64>, title : &str,
                      width: usize, height: usize, padding: usize) -> Graph {
  let mut graph = Graph::new(title.into(), "#8ff0a4".into());
  graph.size = xs.len();
  graph.width = width;
  graph.height = height;
  graph.padding = padding;
  for i in 0..graph.size {
    graph.add_point(xs[i], ys[i]);
  }
  return graph;
} 

fn read_data(csv_str: &str) -> Vec<f64> {
  let split = csv_str.split(",");
  let mut data: Vec<f64> = Vec::new();
  for s in split {
    let value = f64::from_str(s);
    data.push(value.unwrap());
  }
  return data;
}
