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
use rm::learning::optim::grad_desc::GradientDesc;
use rm::learning::glm::{GenLinearModel, Bernoulli};
use rm::learning::gmm::{CovOption, GaussianMixtureModel};
use rm::learning::naive_bayes::{NaiveBayes, Gaussian};
use rm::learning::k_means::KMeansClassifier;
use rm::learning::k_means::KPlusPlus;
use rm::learning::svm::SVM;
use rm::learning::toolkit::kernel::Linear;
use rm::learning::nnet::{NeuralNet, BCECriterion};
use rm::learning::toolkit::activ_fn::Sigmoid;
use rm::learning::toolkit::regularization::Regularization;
use rm::learning::optim::grad_desc::StochasticGD;
use rm::learning::dbscan::DBSCAN;
use rusty_machine::learning::pca::PCA;
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

  pub fn lin_reg(&self, model: &str) -> String {
    let mut context = self.create_svg_context();
    let lin_mod: LinRegressor = serde_json::from_str(model).unwrap();

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

  pub fn log_reg(&self, model: &str) -> String {
    let mut context = self.create_svg_context();
    let log_mod: LogisticRegressor<GradientDesc> = serde_json::from_str(model).unwrap();

    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }
    let inputs = Matrix::new(self.size, 2, p_vec);
    let preds: Vec<f64> = log_mod.predict(&inputs).unwrap().into_vec();

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

  pub fn glm(&self, model: &str) -> String {
    let mut context = self.create_svg_context();
    let gl_mod: GenLinearModel<Bernoulli> = serde_json::from_str(model).unwrap();

    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }
    let inputs = Matrix::new(self.size, 2, p_vec);
    let preds: Vec<f64> = gl_mod.predict(&inputs).unwrap().into_vec();

    println!("glm results: {:?}", gl_mod);
    // println!("glm classification: {:?}", preds);
    context.insert("n", &self.size);
    context.insert("preds", &preds);

    Tera::one_off(include_str!("glm.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn kmeans(&self, model: &str) -> String {
    let mut context = self.create_svg_context();
    let km: KMeansClassifier<KPlusPlus> = serde_json::from_str(model).unwrap();

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

  pub fn svm(&self, model: &str) -> String {
    let mut context = self.create_svg_context();
    let svm_mod: SVM<Linear> = serde_json::from_str(model).unwrap();

    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }
    let inputs = Matrix::new(self.size, 2, p_vec);
    let preds: Vec<f64> = svm_mod.predict(&inputs).unwrap().into_vec();

    context.insert("n", &self.size);
    context.insert("preds", &preds);

    Tera::one_off(include_str!("svm.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn gmm(&self, model: &str) -> String {
    let mut context = self.create_svg_context();
    let gm: GaussianMixtureModel = serde_json::from_str(model).unwrap();

    let mean_mat: Option<&Matrix<f64>> = gm.means();
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

  pub fn dbscan(&self, model: &str) -> String {
    let mut context = self.create_svg_context();
    //assure no more than 5 clusters
    let extra_colors = &["yellow", "purple", "orange", "pink"];
    let db: DBSCAN = serde_json::from_str(model).unwrap();

    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    for point in &self.points {
      xs.push(point.x);
      ys.push(point.y);
    }

    let clustering = db.clusters().unwrap();
    // println!("{:?}", clustering);
    let classes: Vec<i32> = clustering.data().to_vec().iter().map(|&val| match val {Some(x) => {x as i32}, _ => {-1}}).collect();
    let mut clusters: Vec<(f64, f64, usize)> = Vec::new();

    for i in 0..self.size {
      if classes[i] >= 0 {
        if classes[i] >= clusters.len() as i32 {
          for _ in 0..(classes[i] as usize - clusters.len()+1) {
            clusters.push((0.0, 0.0, 0));
          }
        }
        let c_index : usize = classes[i] as usize;
        clusters[c_index] = (clusters[c_index].0+xs[i], clusters[c_index].1+ys[i], clusters[c_index].2+1);
      }
    }
    let mut centers: Vec<(f64, f64)> = clusters.iter().map(|val| (val.0/(val.2 as f64) , val.1/(val.2 as f64)) ).collect();
    centers = self.graph_map(centers);

    let mut context = self.create_svg_context();
    //assure no more than 5 clusters
    let extra_colors = &["yellow", "purple", "orange", "pink"];

    context.insert("n", &self.size);
    context.insert("classes", &classes);
    context.insert("centers", &centers);
    context.insert("num_cluster", &centers.len());
    context.insert("extra_colors", &extra_colors);

    Tera::one_off(include_str!("dbscan.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn pca(&self, model: &str) -> String {
    let mut context = self.create_svg_context();
    let pca: PCA = serde_json::from_str(model).unwrap();

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

#[wasm_bindgen]
pub fn lin_reg_train (csv_content: &[u8]) -> String {
  let data: Vec<f64> = read_data(csv_content);
  let mut xs: Vec<f64> = Vec::new();
  let mut ys: Vec<f64> = Vec::new();
  for i in 0..data.len() {
    if (i % 2) == 1 {
      xs.push(data[i-1]);
      ys.push(data[i]);
    }
  }

  let inputs = Matrix::new(xs.len(), 1, xs);
  let targets = Vector::new(ys);
  let mut lin_mod = LinRegressor::default();
  lin_mod.train(&inputs, &targets).unwrap();

  return serde_json::to_string(&lin_mod).unwrap();
}

#[wasm_bindgen]
pub fn lin_reg_svg (csv_content: &[u8], model: &str) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Linear Regression");
  graph.lin_reg(model)
}

#[wasm_bindgen]
pub fn log_reg_train (csv_content: &[u8]) -> String {
  let data: Vec<f64> = read_data(csv_content);
  let mut p_vec: Vec<f64> = Vec::new();
  let mut labels: Vec<f64> = Vec::new();
  for i in 0..data.len() {
    if (i % 3) == 2 {
      p_vec.push(data[i-2]);
      p_vec.push(data[i-1]);
      labels.push(data[i]);
    }
  }

  let inputs = Matrix::new(labels.len(), 2, p_vec);
  let targets = Vector::new(labels.clone());
  let mut log_mod = LogisticRegressor::default();
  log_mod.train(&inputs, &targets).unwrap();

  return serde_json::to_string(&log_mod).unwrap();
}

#[wasm_bindgen]
pub fn log_reg_svg (csv_content: &[u8], model: &str) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Logistic Regression");
  graph.log_reg(model)
}


#[wasm_bindgen]
pub fn glm_train (csv_content: &[u8]) -> String {
  let data: Vec<f64> = read_data(csv_content);
  let mut p_vec: Vec<f64> = Vec::new();
  let mut labels: Vec<f64> = Vec::new();
  for i in 0..data.len() {
    if (i % 3) == 2 {
      p_vec.push(data[i-2]);
      p_vec.push(data[i-1]);
      labels.push(data[i]);
    }
  }

  let inputs = Matrix::new(labels.len(), 2, p_vec);
  let targets = Vector::new(labels.clone());
  let mut gl_mod = GenLinearModel::new(Bernoulli);
  gl_mod.train(&inputs, &targets).unwrap();

  return serde_json::to_string(&gl_mod).unwrap();
}

#[wasm_bindgen]
pub fn glm_svg (csv_content: &[u8], model: &str) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Generalized Linear Models");
  graph.glm(model)
}

#[wasm_bindgen]
pub fn kmeans_train (csv_content: &[u8], num_centers: i32) -> String {
  let data: Vec<f64> = read_data(csv_content);
  let inputs = Matrix::new(data.len()/2, 2, data);
  let mut km = KMeansClassifier::new(num_centers as usize);
  km.train(&inputs).unwrap();

  return serde_json::to_string(&km).unwrap();
}

#[wasm_bindgen]
pub fn kmeans_svg (csv_content: &[u8], model: &str) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "K-Means Clustering");
  graph.kmeans(model)
}

#[wasm_bindgen]
pub fn svm_train (csv_content: &[u8]) -> String {
  let data: Vec<f64> = read_data(csv_content);
  let mut p_vec: Vec<f64> = Vec::new();
  let mut labels: Vec<f64> = Vec::new();
  for i in 0..data.len() {
    if (i % 3) == 2 {
      p_vec.push(data[i-2]);
      p_vec.push(data[i-1]);
      labels.push(data[i]);
    }
  }

  let inputs = Matrix::new(labels.len(), 2, p_vec);
  let targets = Vector::new(labels);
  let mut svm_mod = SVM::new(Linear::default(), 0.2);
  svm_mod.train(&inputs, &targets).unwrap();

  return serde_json::to_string(&svm_mod).unwrap();
}

#[wasm_bindgen]
pub fn svm_svg (csv_content: &[u8], model: &str) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Support Vector Machine");
  graph.svm(model)
}

#[wasm_bindgen]
pub fn gmm_train (csv_content: &[u8], num_centers: i32) -> String {
  let data: Vec<f64> = read_data(csv_content);
  let inputs = Matrix::new(data.len()/2, 2, data);

  let mut gm = GaussianMixtureModel::new(num_centers as usize);
  gm.set_max_iters(10);
  gm.cov_option = CovOption::Diagonal;
  gm.train(&inputs).unwrap();

  return serde_json::to_string(&gm).unwrap();
}

#[wasm_bindgen]
pub fn gmm_svg (csv_content: &[u8], model: &str) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Gaussian Mixture Model");
  graph.gmm(model)
}

#[wasm_bindgen]
pub fn dbscan_train (csv_content: &[u8]) -> String {
  let data: Vec<f64> = read_data(csv_content);
  let inputs = Matrix::new(data.len()/2, 2, data);
  let mut db = DBSCAN::new(0.5, 2);
  db.train(&inputs).unwrap();

  return serde_json::to_string(&db).unwrap();
}

#[wasm_bindgen]
pub fn dbscan_svg (csv_content: &[u8], model: &str) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "DBSCAN");
  graph.dbscan(model)
}

#[wasm_bindgen]
pub fn pca_train (csv_content: &[u8]) -> String {
  let data: Vec<f64> = read_data(csv_content);
  let inputs = Matrix::new(data.len()/2, 2, data);
  let mut pca = PCA::default();
  pca.train(&inputs).unwrap();

  return serde_json::to_string(&pca).unwrap();
}

#[wasm_bindgen]
pub fn pca_svg (csv_content: &[u8], model: &str) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Principal Components Analysis");
  graph.pca(model)
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
