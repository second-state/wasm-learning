use wasm_bindgen::prelude::*;
use tera::{Context, Tera};
use std::str::FromStr;
use std::iter::Iterator;
use std::string::String;
use core::option::Option;

extern crate rusty_machine as rm;
use rm::linalg::Matrix;
use rm::linalg::Vector;
use rm::learning::lin_reg::LinRegressor;
use rm::learning::logistic_reg::LogisticRegressor;
use rm::learning::glm::{GenLinearModel, Bernoulli};
use rm::learning::gmm::{CovOption, GaussianMixtureModel};
use rm::learning::naive_bayes::{NaiveBayes, Gaussian};
use rm::learning::k_means::KMeansClassifier;
use rm::learning::svm::SVM;
use rm::learning::toolkit::kernel::Linear;
use rm::learning::nnet::{NeuralNet, BCECriterion};
use rm::learning::toolkit::activ_fn::Sigmoid;
use rm::learning::toolkit::regularization::Regularization;
use rm::learning::optim::grad_desc::StochasticGD;
use rm::learning::dbscan::DBSCAN;
use rm::learning::SupModel;
use rm::learning::UnSupModel;

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

  pub fn lin_reg_svg(&self) -> String {
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    for point in &self.points {
      xs.push(point.x);
      ys.push(point.y);
    }

    let mut context = self.create_svg_context();

    let inputs = Matrix::new(self.size, 1, xs);
    let targets = Vector::new(ys);
    let mut lin_mod = LinRegressor::default();
    lin_mod.train(&inputs, &targets).unwrap();
    let params : Option<&Vector<f64>> = lin_mod.parameters();
    let mut coefs : Vec<f64> = Vec::new();
    let mut ps : Vec<(f64, f64)> = Vec::new();

    if params.is_some() {
      // println!("{}", params.unwrap().size());
      for i in 0..params.unwrap().size() {
        coefs.push(params.unwrap()[i]);
      }
    }
    if coefs.len() > 0 {
      ps.push((self.x_min, coefs[0] + coefs[1] * self.x_min));
      ps.push((self.x_min + self.x_range, coefs[0] + coefs[1] * (self.x_min + self.x_range)));
      ps = self.graph_map(ps);
    }
    // println!("{:?}", type_of(p1));
    // println!("{:?}", p2);
    println!("lin_reg results: {:?}", coefs);
    context.insert("point1", &(ps[0]));
    context.insert("point2", &(ps[1])); 

    Tera::one_off(include_str!("lin_reg.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn log_reg_svg(&self) -> String {
    let target_vec: Vec<f64> = get_label(self.size);
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }

    let mut context = self.create_svg_context();

    let inputs = Matrix::new(self.size, 2, p_vec);
    let targets = Vector::new(target_vec);
    let mut log_mod = LogisticRegressor::default();
    log_mod.train(&inputs, &targets).unwrap();
    
    let params : Option<&Vector<f64>> = log_mod.parameters();
    let mut coefs : Vec<f64> = Vec::new();
    let mut ps : Vec<(f64, f64)> = Vec::new();
    if params.is_some() {
      // println!("{:?}", params.unwrap());
      coefs.push(-params.unwrap()[0]/params.unwrap()[2]);
      coefs.push(-params.unwrap()[1]/params.unwrap()[2]);
    }

    if coefs.len() > 0 {
      ps.push((self.x_min, coefs[0] + coefs[1] * self.x_min));
      ps.push((self.x_min + self.x_range, coefs[0] + coefs[1] * (self.x_min + self.x_range)));
      ps = self.graph_map(ps);
    }

    let preds: Vec<f64> = log_mod.predict(&inputs).unwrap().into_vec();

    // println!("{:?}", type_of(p1));
    // println!("{:?}", p2);
    println!("log_reg results: {:?}", coefs);
    // println!("log_reg classification: {:?}", preds);
    context.insert("point1", &(ps[0]));
    context.insert("point2", &(ps[1]));
    context.insert("n", &self.size);
    context.insert("preds", &preds);

    Tera::one_off(include_str!("log_reg.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn glm_svg(&self) -> String {
    let target_vec: Vec<f64> = get_label(self.size);
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }

    let mut context = self.create_svg_context();

    let inputs = Matrix::new(self.size, 2, p_vec);
    let targets = Vector::new(target_vec);
    let mut gl_mod = GenLinearModel::new(Bernoulli);
    gl_mod.train(&inputs, &targets).unwrap();
    
    let preds: Vec<f64> = gl_mod.predict(&inputs).unwrap().into_vec();

    println!("glm results: {:?}", gl_mod);
    // println!("glm classification: {:?}", preds);
    context.insert("n", &self.size);
    context.insert("preds", &preds);

    Tera::one_off(include_str!("glm.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn kmeans_svg(&self, num_centers: usize) -> String {
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }

    let mut context = self.create_svg_context();

    // let center_num : usize = 2;
    let inputs = Matrix::new(self.size, 2, p_vec);
    let mut km = KMeansClassifier::new(num_centers);
    km.train(&inputs).unwrap();
    let center_mat = km.centroids().as_ref().unwrap();
    
    let center_vec: Vec<f64> = center_mat.data().to_vec();
    let mut centers: Vec<(f64, f64)> = Vec::new();
    for i in 0..center_vec.len() {
      if (i % 2) == 1 {
        centers.push((center_vec[i-1], center_vec[i]));
      } 
    }
    centers = self.graph_map(centers);
    println!("kmeans results: {:?}", centers);

    context.insert("centers", &centers);
    Tera::one_off(include_str!("kmeans.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn nnet_svg(&self) -> String {
    let target_vec: Vec<f64> = get_label(self.size);
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }

    let mut context = self.create_svg_context();

    let inputs = Matrix::new(self.size, 2, p_vec);
    let mut target_class: Vec<f64> = Vec::new();
    for i in 0..self.size {
      if target_vec[i] == 0. {
        target_class.push(1.);
        target_class.push(0.);
      } else {
        target_class.push(0.);
        target_class.push(1.);
      }
    }
    let targets = Matrix::new(self.size, 2, target_class);
    let layers = &[2,5,11,7,2];
    let criterion = BCECriterion::new(Regularization::L2(0.1));
    println!("Criterion created!");
    let mut nn = NeuralNet::mlp(layers, criterion, StochasticGD::default(), Sigmoid);
    println!("Net not trained");
    nn.train(&inputs, &targets).unwrap();
    let pred_class: Vec<f64> = nn.predict(&inputs).unwrap().into_vec();
    let mut preds: Vec<f64> = Vec::new();
    for i in 0..self.size {
      if pred_class[2*i] <= 0.5 {
        preds.push(1.);
      } else {
        preds.push(0.);
      }
    }
    context.insert("n", &self.size);
    context.insert("preds", &preds);
    println!("nnet results: {:?}", nn);
    // println!("nnet classification: {:?}", preds);

    Tera::one_off(include_str!("nnet.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn svm_svg(&self) -> String {
    let target_vec: Vec<f64> = get_label(self.size);
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }

    let mut context = self.create_svg_context();

    let svm_target_vec: Vec<f64> = target_vec.iter().map(|val| if *val == 1.0 as f64 {1. as f64} else {-1. as f64} ).collect();
    let inputs = Matrix::new(self.size, 2, p_vec);
    let targets = Vector::new(svm_target_vec);
    let mut svm_mod = SVM::new(Linear::default(), 0.2);
    svm_mod.train(&inputs, &targets).unwrap();
    
    let preds: Vec<f64> = svm_mod.predict(&inputs).unwrap().into_vec();
    println!("svm results: {:?}", svm_mod);
    // println!("svm classification: {:?}", preds);

    context.insert("n", &self.size);
    context.insert("preds", &preds);

    Tera::one_off(include_str!("svm.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn gmm_svg(&self, num_classes: usize) -> String {
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }

    let mut context = self.create_svg_context();

    // let class_num: usize = 2;
    let inputs = Matrix::new(self.size, 2, p_vec);
    let mut gm = GaussianMixtureModel::new(num_classes);
    gm.set_max_iters(10);
    gm.cov_option = CovOption::Diagonal;
    gm.train(&inputs).unwrap();

    let mean_mat: Option<&Matrix<f64>> = gm.means();
    // if mean_mat.is_some() {
    //   println!("{:?}", mean_mat.unwrap());
    // }

    let mean_vec: Vec<f64> = mean_mat.unwrap().data().to_vec();
    let mut mus: Vec<(f64, f64)> = Vec::new();

    for i in 0..mean_vec.len() {
      if (i % 2) == 1 {
        mus.push((mean_vec[i-1], mean_vec[i]));
      } 
    }
    mus = self.graph_map(mus);
    println!("gmm results: {:?}", mus);

    context.insert("means", &mus);

    Tera::one_off(include_str!("gmm.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn nb_svg(&self) -> String {
    let target_vec: Vec<f64> = get_label(self.size);
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }

    let mut context = self.create_svg_context();

    let inputs = Matrix::new(self.size, 2, p_vec);
    let mut target_class: Vec<f64> = Vec::new();
    for i in 0..self.size {
      if target_vec[i] == 0. {
        target_class.push(1.);
        target_class.push(0.);
      } else {
        target_class.push(0.);
        target_class.push(1.);
      }
    }
    let targets = Matrix::new(self.size, 2, target_class);
    let mut nb = NaiveBayes::<Gaussian>::new();
    nb.train(&inputs, &targets).unwrap();
    let pred_class: Vec<f64> = nb.predict(&inputs).unwrap().into_vec();
    let mut preds: Vec<f64> = Vec::new();
    for i in 0..self.size {
      if pred_class[2*i] == 0. {
        preds.push(1.);
      } else {
        preds.push(0.)
      }
    }

    // println!("nb results: {:?}", nb);
    // println!("nb classification: {:?}", preds);

    context.insert("n", &self.size);
    context.insert("preds", &preds);

    Tera::one_off(include_str!("nb.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn dbscan_svg(&self) -> String {
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      xs.push(point.x);
      ys.push(point.y);
      p_vec.push(point.x);
      p_vec.push(point.y);
    }

    let mut context = self.create_svg_context();

    let inputs = Matrix::new(self.size, 2, p_vec);
    let mut db = DBSCAN::new(0.5, 2);
    db.train(&inputs).unwrap();

    let clustering = db.clusters().unwrap();
    let labels: Vec<f64> = clustering.data().to_vec().iter().map(|&val| match val {Some(x) => {x as f64}, _ => {-1.0}}).collect();
    let mut clusters: Vec<(f64, f64, usize)> = Vec::new(); 
    
    for i in 0..self.size {
      if labels[i] >= 0.0 {
        if labels[i] >= clusters.len() as f64 {
          for _ in 0..(labels[i] as usize - clusters.len()+1) {
            clusters.push((0.0, 0.0, 0));
          }
        }
        let c_index : usize = labels[i] as usize;
        clusters[c_index] = (clusters[c_index].0+xs[i], clusters[c_index].1+ys[i], clusters[c_index].2+1);
      }
    }
    let mut centers: Vec<(f64, f64)> = clusters.iter().map(|val| (val.0/(val.2 as f64) , val.1/(val.2 as f64)) ).collect(); 
    centers = self.graph_map(centers);

    println!("dbscan results: {:?}", centers);

    context.insert("n", &self.size);
    context.insert("labels", &labels);
    context.insert("centers", &centers);

    Tera::one_off(include_str!("dbscan.svg"), &mut context, true).expect("Could not draw graph")
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


#[wasm_bindgen]
pub fn lin_reg (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Linear Regression");
  graph.lin_reg_svg()
}

#[wasm_bindgen]
pub fn log_reg (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Logistic Regression");
  graph.log_reg_svg()
}

#[wasm_bindgen]
pub fn glm (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Generalized Linear Models");
  graph.glm_svg()
}

#[wasm_bindgen]
pub fn kmeans (csv_content: &[u8], num_centers: i32) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "K-Means Clustering");
  graph.kmeans_svg(num_centers as usize)
}

#[wasm_bindgen]
pub fn nnet (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Neural Networks");
  graph.nnet_svg()
}

#[wasm_bindgen]
pub fn svm (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Support Vector Machines");
  graph.svm_svg()
}

#[wasm_bindgen]
pub fn gmm (csv_content: &[u8], num_classes: i32) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Gaussian Mixture Models");
  graph.gmm_svg(num_classes as usize)
}

#[wasm_bindgen]
pub fn nb (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Naive Bayes Classifiers");
  graph.nb_svg()
}

#[wasm_bindgen]
pub fn dbscan (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "DBSCAN");
  graph.dbscan_svg()
}

pub fn prepare_graph (csv_content: &[u8], width: usize, height: usize, padding: usize, title: &str) -> Graph {
  let data: Vec<f64> = read_data(csv_content);
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

fn read_data(csv_content: &[u8]) -> Vec<f64> {
  let mut data_reader = csv::Reader::from_reader(csv_content);
  let mut data: Vec<f64> = Vec::new();
  for record in data_reader.records() {
    for field in record.unwrap().iter() {
      let value = f64::from_str(field);
      data.push(value.unwrap());
    }
  }
  return data;
}

fn get_label(n : usize) -> Vec<f64> {
  let mut target_vec: Vec<f64> = Vec::new();
  for i in 0..n {
    if i < n/2 {
      target_vec.push(0.);
    } else {
      target_vec.push(1.);
    }
  }
  target_vec
}
