use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
// use std::io::prelude::*;

// use serde_json;
use tera::{Context, Tera};
use std::str::FromStr;
use std::iter::Iterator;
// use std::any::type_name;
use std::string::String;
use core::option::Option;

extern crate rusty_machine as rm;
// use rm::prelude::*;
use rm::linalg::Matrix;
use rm::linalg::Vector;
// use rm::learning::LearningResult;
// use rulinalg::vector::Vector;
use rm::learning::lin_reg::LinRegressor;


use rm::learning::logistic_reg::LogisticRegressor;
use rm::learning::optim::grad_desc::GradientDesc;

use rm::learning::glm::{GenLinearModel, Bernoulli};

use rm::learning::gmm::{CovOption, GaussianMixtureModel};

use rm::learning::naive_bayes::{NaiveBayes, Gaussian};

use rm::learning::k_means::KMeansClassifier;
use rm::learning::k_means::KPlusPlus;

use rm::learning::svm::SVM;
// use rm::learning::toolkit::kernel::Linear;

use rm::learning::nnet::{NeuralNet, BCECriterion};
use rm::learning::toolkit::activ_fn::Sigmoid;
use rm::learning::toolkit::regularization::Regularization;
use rm::learning::optim::grad_desc::StochasticGD;

use rm::learning::dbscan::DBSCAN;

use rm::learning::pca::PCA;

use rm::learning::gp;
use rm::learning::gp::GaussianProcess;
use rm::learning::toolkit::kernel::SquaredExp;
use rm::learning::gp::ConstMean;

use rm::learning::SupModel;
use rm::learning::UnSupModel;


#[derive(Serialize, Deserialize, Clone, Debug)]
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
    pub labels: Vec<f64>,
    pub attributes: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
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
          labels: Vec::new(),
          attributes: 0,
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

  pub fn train_lin_reg(&self) -> LinRegressor {
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    for point in &self.points {
      xs.push(point.x);
      ys.push(point.y);
    }


    let inputs = Matrix::new(self.size, 1, xs);
    let targets = Vector::new(ys);
    let mut lin_mod = LinRegressor::default();
    lin_mod.train(&inputs, &targets).unwrap();

    return lin_mod;
  }

  pub fn lin_reg_svg(&self, model: Option<&str>) -> String {
    /* let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    for point in &self.points {
      xs.push(point.x);
      ys.push(point.y);
    }*/
    let mut lin_mod: LinRegressor = serde_json::from_str(model.unwrap()).unwrap();
    if !model.is_some() {
      lin_mod = self.train_lin_reg();
    }

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
      if (coefs[1] < 1.0) && (coefs[1] > -1.0) {
        ps.push((self.x_min, coefs[0] + coefs[1] * self.x_min));
        ps.push((self.x_min + self.x_range, coefs[0] + coefs[1] * (self.x_min + self.x_range)));
      } else {
        ps.push(((self.y_min-coefs[0]) / coefs[1], self.y_min));
        ps.push(((self.y_min+self.y_range-coefs[0]) / coefs[1], self.y_min+self.y_range));
      }
      ps = self.graph_map(ps);
    }
    // println!("{:?}", type_of(p1));
    // println!("{:?}", p2);
    let mut context = self.create_svg_context();
    context.insert("point1", &(ps[0]));
    context.insert("point2", &(ps[1])); 

    Tera::one_off(include_str!("lin_reg.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn train_log_reg(&self) -> LogisticRegressor<GradientDesc> {
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }

    let inputs = Matrix::new(self.size, 2, p_vec);
    let targets = Vector::new(self.labels.clone());
    let mut log_mod = LogisticRegressor::default();
    log_mod.train(&inputs, &targets).unwrap();

    return log_mod;
  }

  pub fn log_reg_svg(&self, model: Option<&str>) -> String {
    let mut log_mod: LogisticRegressor<GradientDesc> = serde_json::from_str(model.unwrap()).unwrap();
    if !model.is_some() {
      log_mod = self.train_log_reg();
    }

    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }

    let inputs = Matrix::new(self.size, 2, p_vec);
    // let targets = Vector::new(self.labels.clone());

    let params : Option<&Vector<f64>> = log_mod.parameters();
    let mut coefs : Vec<f64> = Vec::new();
    let mut ps : Vec<(f64, f64)> = Vec::new();
    if params.is_some() {
      // println!("{:?}", params.unwrap());
      coefs.push(-params.unwrap()[0]/params.unwrap()[2]);
      coefs.push(-params.unwrap()[1]/params.unwrap()[2]);
    }

    if coefs.len() > 0 {
      if (coefs[1] < 1.0) && (coefs[1] > -1.0) {
        ps.push((self.x_min, coefs[0] + coefs[1] * self.x_min));
        ps.push((self.x_min + self.x_range, coefs[0] + coefs[1] * (self.x_min + self.x_range)));
      } else {
        ps.push(((self.y_min-coefs[0]) / coefs[1], self.y_min));
        ps.push(((self.y_min+self.y_range-coefs[0]) / coefs[1], self.y_min+self.y_range));
      }
      ps = self.graph_map(ps);
    }

    let preds: Vec<f64> = log_mod.predict(&inputs).unwrap().into_vec();

    // println!("{:?}", type_of(p1));
    // println!("{:?}", p2);
    let mut context = self.create_svg_context();
    context.insert("point1", &(ps[0]));
    context.insert("point2", &(ps[1]));
    context.insert("n", &self.size);
    context.insert("preds", &preds);

    Tera::one_off(include_str!("log_reg.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn train_glm(&self) -> GenLinearModel<Bernoulli> {
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }


    let inputs = Matrix::new(self.size, 2, p_vec);
    let targets = Vector::new(self.labels.clone());

    let mut gl_mod = GenLinearModel::new(Bernoulli);
    gl_mod.train(&inputs, &targets).unwrap();

    return gl_mod;
  }

  pub fn glm_svg(&self, model: Option<&str>) -> String {
    let mut gl_mod: GenLinearModel<Bernoulli> = serde_json::from_str(model.unwrap()).unwrap();
    if !model.is_some() {
      gl_mod = self.train_glm();
    }

    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }


    let inputs = Matrix::new(self.size, 2, p_vec);
    // let targets = Vector::new(self.labels.clone());
    
    /*let params : Option<&Vector<f64>> = log_mod.parameters();
    let mut coefs : Vec<f64> = Vec::new();
    let mut p1 : (f64, f64) = (0.0, 0.0);
    let mut p2 : (f64, f64) = (0.0, 0.0);
    if params.is_some() {
      // println!("{:?}", params.unwrap());
      coefs.push(-params.unwrap()[0]/params.unwrap()[2]);
      coefs.push(-params.unwrap()[1]/params.unwrap()[2]);
    }
    if coefs.len() > 0 {
      p1 = (self.x_min, coefs[0] + coefs[1] * self.x_min);
      p2 = (self.x_min + self.x_range, coefs[0] + coefs[1] * (self.x_min + self.x_range));
      p1 = ((p1.0 - self.x_min) / self.x_range * width as f64 + padding as f64, 
                (p1.1 - self.y_min) / self.y_range * (height as f64 * -1.0) + (padding + height) as f64);
      p2 = ((p2.0 - self.x_min) / self.x_range * width as f64 + padding as f64, 
                (p2.1 - self.y_min) / self.y_range * (height as f64 * -1.0) + (padding + height) as f64);
    }*/
    let preds: Vec<f64> = gl_mod.predict(&inputs).unwrap().into_vec();

    // println!("{:?}", type_of(p1));
    // println!("{:?}", p2);
    let mut context = self.create_svg_context();
    // context.insert("point1", &p1);
    // context.insert("point2", &p2);
    context.insert("n", &self.size);
    context.insert("preds", &preds);

    Tera::one_off(include_str!("glm.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn train_kmeans(&self, num_centers: usize) -> KMeansClassifier<KPlusPlus> {
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }

    // let center_num : usize = 2;
    let inputs = Matrix::new(self.size, 2, p_vec);
    let mut km = KMeansClassifier::new(num_centers);
    km.train(&inputs).unwrap();

    return km;
  }


  pub fn kmeans_svg(&self, num_centers: usize, model: Option<&str>) -> String {
    let mut km: KMeansClassifier<KPlusPlus> = serde_json::from_str(model.unwrap()).unwrap();
    if !model.is_some() {
      km = self.train_kmeans(num_centers);
    }
    // let center_num : usize = 2;
    let center_mat = km.centroids().as_ref().unwrap();
    
    let center_vec: Vec<f64> = center_mat.data().to_vec();
    let mut centers: Vec<(f64, f64)> = Vec::new();
    for i in 0..center_vec.len() {
      if (i % 2) == 1 {
        centers.push((center_vec[i-1], center_vec[i]));
      } 
    }

    let mut context = self.create_svg_context();
    centers = self.graph_map(centers);
    context.insert("centers", &centers);
    Tera::one_off(include_str!("kmeans.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn train_nnet(&self) -> NeuralNet<BCECriterion, StochasticGD> {
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }


    let inputs = Matrix::new(self.size, 2, p_vec);
    let mut target_class: Vec<f64> = Vec::new();
    for i in 0..self.size {
      if self.labels[i] == 0. {
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
    let mut nn = NeuralNet::mlp(layers, criterion, StochasticGD::default(), Sigmoid);
    nn.train(&inputs, &targets).unwrap();

    return nn;
  }

  pub fn nnet_svg(&self) -> String {
    let nn = self.train_nnet();

    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }
    let inputs = Matrix::new(self.size, 2, p_vec);

    let pred_class: Vec<f64> = nn.predict(&inputs).unwrap().into_vec();
    let mut preds: Vec<f64> = Vec::new();
    for i in 0..self.size {
      if pred_class[2*i] <= 0.5 {
        preds.push(1.);
      } else {
        preds.push(0.);
      }
    }
    let mut context = self.create_svg_context();
    context.insert("n", &self.size);
    context.insert("preds", &preds);
    // println!("{:?}", preds);

    Tera::one_off(include_str!("nnet.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn train_svm(&self) -> SVM<SquaredExp> {
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }

    let svm_target_vec: Vec<f64> = self.labels.iter().map(|val| if *val == 1.0 as f64 {1. as f64} else {-1. as f64} ).collect(); 
    println!("{:?}", svm_target_vec);    
    let inputs = Matrix::new(self.size, 2, p_vec);
    let targets = Vector::new(svm_target_vec);
    // println!("Nothing yet!");
    let mut svm_mod = SVM::new(SquaredExp::default(), 0.3);
    svm_mod.train(&inputs, &targets).unwrap();
    return svm_mod;
  }

  pub fn svm_svg(&self, model: Option<&str>) -> String {
    let mut svm_mod: SVM<SquaredExp> = serde_json::from_str(model.unwrap()).unwrap();
    if !model.is_some() {
      svm_mod = self.train_svm();
    }

    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }
         
    let inputs = Matrix::new(self.size, 2, p_vec);
    
    /*let params : Option<&Vector<f64>> = log_mod.parameters();
    let mut coefs : Vec<f64> = Vec::new();
    let mut p1 : (f64, f64) = (0.0, 0.0);
    let mut p2 : (f64, f64) = (0.0, 0.0);
    if params.is_some() {
      // println!("{:?}", params.unwrap());
      coefs.push(-params.unwrap()[0]/params.unwrap()[2]);
      coefs.push(-params.unwrap()[1]/params.unwrap()[2]);
    }
    if coefs.len() > 0 {
      p1 = (self.x_min, coefs[0] + coefs[1] * self.x_min);
      p2 = (self.x_min + self.x_range, coefs[0] + coefs[1] * (self.x_min + self.x_range));
      p1 = ((p1.0 - self.x_min) / self.x_range * width as f64 + padding as f64, 
                (p1.1 - self.y_min) / self.y_range * (height as f64 * -1.0) + (padding + height) as f64);
      p2 = ((p2.0 - self.x_min) / self.x_range * width as f64 + padding as f64, 
                (p2.1 - self.y_min) / self.y_range * (height as f64 * -1.0) + (padding + height) as f64);
    }*/
    // println!("{:?}", svm_mod.predict(&inputs).unwrap());
    let preds: Vec<f64> = svm_mod.predict(&inputs).unwrap().into_vec();
    println!("{:?}", preds);
    // preds = preds.iter().map(|val| if *val == 1.0 as f64 {1.} else {0.}).collect();
    // println!("{:?}", type_of(p1));
    // println!("{:?}", p2);
    //context.insert("point1", &p1);
    //context.insert("point2", &p2);
    let mut context = self.create_svg_context();
    context.insert("n", &self.size);
    context.insert("preds", &preds);

    Tera::one_off(include_str!("svm.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn train_gmm(&self, class_num: usize) -> GaussianMixtureModel {
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }


    let inputs = Matrix::new(self.size, 2, p_vec);
    // println!("{:?}", inputs);
    let mut gm = GaussianMixtureModel::new(class_num);
    gm.set_max_iters(10);
    gm.cov_option = CovOption::Diagonal;
    gm.train(&inputs).unwrap();
    return gm;
  }

  pub fn gmm_svg(&self, class_num: usize, model: Option<&str>) -> String {
    let mut gm: GaussianMixtureModel = serde_json::from_str(model.unwrap()).unwrap();
    if !model.is_some() {
      gm = self.train_gmm(class_num);
    }

    let mean_mat: Option<&Matrix<f64>> = gm.means();
    if mean_mat.is_some() {
      println!("{:?}", mean_mat.unwrap());
    }

    let mean_vec: Vec<f64> = mean_mat.unwrap().data().to_vec();
    let mut mus: Vec<(f64, f64)> = Vec::new();

    for i in 0..mean_vec.len() {
      if (i % 2) == 1 {
        mus.push((mean_vec[i-1], mean_vec[i]));
      } 
    }
    mus = self.graph_map(mus);

    let mut context = self.create_svg_context();    
    context.insert("means", &mus);

    Tera::one_off(include_str!("gmm.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn train_nb(&self) -> NaiveBayes<Gaussian> {
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }


    let inputs = Matrix::new(self.size, 2, p_vec);
    let mut target_class: Vec<f64> = Vec::new();
    for i in 0..self.size {
      if self.labels[i] == 0. {
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

    return nb;
  }

  pub fn nb_svg(&self, model: Option<&str>) -> String {
    let mut nb: NaiveBayes<Gaussian> = serde_json::from_str(model.unwrap()).unwrap();
    if !model.is_some() {
      nb = self.train_nb();
    }

    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }
    let inputs = Matrix::new(self.size, 2, p_vec);
    
    let pred_class: Vec<f64> = nb.predict(&inputs).unwrap().into_vec();
    let mut preds: Vec<f64> = Vec::new();
    for i in 0..self.size {
      if pred_class[2*i] == 0. {
        preds.push(1.);
      } else {
        preds.push(0.);
      }
    }

    // println!("{:?}", type_of(p1));
    // println!("{:?}", preds);
    let mut context = self.create_svg_context();
    context.insert("n", &self.size);
    context.insert("preds", &preds);

    Tera::one_off(include_str!("nb.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn train_dbscan(&self) -> DBSCAN {
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }
    let inputs = Matrix::new(self.size, 2, p_vec);
    let mut db = DBSCAN::new(0.5, 2);
    db.train(&inputs).unwrap(); 

    return db;
  }

  pub fn dbscan_svg(&self, model: Option<&str>) -> String {
    let mut db: DBSCAN = serde_json::from_str(model.unwrap()).unwrap();
    if !model.is_some() {
      db = self.train_dbscan();
    }

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

  pub fn train_pca(&self) -> PCA {
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }
    let inputs = Matrix::new(self.size, 2, p_vec);
    let mut pca = PCA::default();
    pca.train(&inputs).unwrap();

    return pca;
  }

  pub fn pca_svg(&self, model: Option<&str>) -> String {
    /* let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    for point in &self.points {
      xs.push(point.x);
      ys.push(point.y);
    }*/
    let mut pca: PCA = serde_json::from_str(model.unwrap()).unwrap();
    if !model.is_some() {
      pca = self.train_pca();
    }

    let graph_center: (f64, f64) = (self.x_min + self.x_range/2.0, self.y_min + self.y_range/2.0);

    let eig_mat = pca.components().unwrap();
    // println!("{:?}", eigvecs);
    let eig_vecs: Vec<f64> = eig_mat.data().to_vec();
    let mut eig_vecs_coefs: Vec<(f64, f64)> = Vec::new();
    for i in 0..(eig_vecs.len()/2) {
      eig_vecs_coefs.push((graph_center.1 - eig_vecs[2*i+1]/eig_vecs[2*i]*graph_center.0, eig_vecs[2*i+1]/eig_vecs[2*i]));
    }

    println!("{:?}", eig_vecs_coefs);

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

    let mut context = self.create_svg_context();
    context.insert("eig_vector1_point1", &(eig_vecs_points[0]));
    context.insert("eig_vector1_point2", &(eig_vecs_points[1]));
    context.insert("eig_vector2_point1", &(eig_vecs_points[2]));
    context.insert("eig_vector2_point2", &(eig_vecs_points[3]));

    Tera::one_off(include_str!("pca.svg"), &mut context, true).expect("Could not draw graph")
  }

  pub fn train_gp(&self) -> GaussianProcess<SquaredExp, ConstMean> {
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }

    let inputs = Matrix::new(self.size, 2, p_vec);
    let targets = Vector::new(self.labels.clone());
    println!("{:?}", targets);

    let mut gaussp = gp::GaussianProcess::default();
    gaussp.train(&inputs, &targets).unwrap();

    return gaussp;
  }

  pub fn gp_svg(&self, model: Option<&str>) -> String {
    let mut p_vec: Vec<f64> = Vec::new();
    for point in &self.points {
      p_vec.push(point.x);
      p_vec.push(point.y);
    }

    let inputs = Matrix::new(self.size, 2, p_vec);

    let mut gaussp: GaussianProcess<SquaredExp, ConstMean> = serde_json::from_str(model.unwrap()).unwrap();
    if !model.is_some() {
      gaussp = self.train_gp();
    }

    let preds = gaussp.predict(&inputs).unwrap().data().to_vec();

    let mut context = self.create_svg_context();
    context.insert("preds", &preds);

    Tera::one_off(include_str!("gp.svg"), &mut context, true).expect("Could not draw graph")
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
    // context.insert("labels", &self.labels);
    context.insert("lines", &10);

    return context;
  }
}

#[wasm_bindgen]
pub fn lin_reg (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Linear Regression");
  let lin_mod = graph.train_lin_reg();

  return serde_json::to_string(&lin_mod).unwrap();
}

#[wasm_bindgen]
pub fn plot_lin_reg (csv_content: &[u8], lin_mod: String, trained: i32) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Linear Regression");
  if trained != 0 {
    graph.lin_reg_svg(Some(lin_mod.as_str()))
  } else {
    graph.lin_reg_svg(None)
  }
}


#[wasm_bindgen]
pub fn log_reg (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Logistic Regression");
  let log_mod = graph.train_log_reg();
  
  return serde_json::to_string(&log_mod).unwrap();
}

#[wasm_bindgen]
pub fn plot_log_reg (csv_content: &[u8], log_mod: String, trained: i32) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Logistic Regression");
  if trained != 0 {
    graph.log_reg_svg(Some(log_mod.as_str()))
  } else {
    graph.log_reg_svg(None)
  }
}


#[wasm_bindgen]
pub fn glm (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Generalized Linear Models");
  let gl_mod = graph.train_glm();
  
  return serde_json::to_string(&gl_mod).unwrap();
}

#[wasm_bindgen]
pub fn plot_glm (csv_content: &[u8], gl_mod: String, trained: i32) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Generalized Linear Models");
  if trained != 0 {
    graph.glm_svg(Some(gl_mod.as_str()))
  } else {
    graph.glm_svg(None)
  }
}

#[wasm_bindgen]
pub fn kmeans (csv_content: &[u8], num_centers: i32) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "K-Means Clustering");
  let km = graph.train_kmeans(num_centers as usize);

  return serde_json::to_string(&km).unwrap();
}

#[wasm_bindgen]
pub fn plot_kmeans (csv_content: &[u8], num_centers: i32, km: String, trained: i32) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "K-Means Clustering");
  if trained != 0 {
    graph.kmeans_svg(num_centers as usize, Some(km.as_str()))
  } else {
    graph.kmeans_svg(num_centers as usize, None)
  }
}

/*
#[wasm_bindgen]
pub fn nnet (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Neural Networks");
  
  let nn = graph.train_nnet();

  return serde_json::to_string(&nn).unwrap();
}*/

#[wasm_bindgen]
pub fn plot_nnet (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Neural Networks");
  graph.nnet_svg()
}

#[wasm_bindgen]
pub fn svm (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Support Vector Machines");
  let svm_mod = graph.train_svm();

  return serde_json::to_string(&svm_mod).unwrap();
}

#[wasm_bindgen]
pub fn plot_svm (csv_content: &[u8], svm: String, trained: i32) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Support Vector Machines");
  if trained != 0 {
    graph.svm_svg(Some(svm.as_str()))
  } else {
    graph.svm_svg(None)
  }
}

#[wasm_bindgen]
pub fn gmm (csv_content: &[u8], class_num: i32) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Gaussian Mixture Models");
  let gm = graph.train_gmm(class_num as usize);

  return serde_json::to_string(&gm).unwrap();
}

#[wasm_bindgen]
pub fn plot_gmm (csv_content: &[u8], class_num: i32, gm: String, trained: i32) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Gaussian Mixture Models");
  if trained != 0 {
    graph.gmm_svg(class_num as usize, Some(gm.as_str()))
  } else {
    graph.gmm_svg(class_num as usize, None)
  }
}

#[wasm_bindgen]
pub fn nb (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Naive Bayes Classifiers");
  let nb = graph.train_nb();

  return serde_json::to_string(&nb).unwrap();
}

#[wasm_bindgen]
pub fn plot_nb (csv_content: &[u8], nb: String, trained: i32) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "Naive Bayes Classifiers");
  if trained != 0 {
    graph.nb_svg(Some(nb.as_str()))
  } else {
    graph.nb_svg(None)
  }
}

#[wasm_bindgen]
pub fn dbscan (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "DBSCAN");
  let db = graph.train_dbscan();

  return serde_json::to_string(&db).unwrap();
}

#[wasm_bindgen]
pub fn plot_dbscan (csv_content: &[u8], dbscan: String, trained: i32) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "DBSCAN");
  if trained != 0 {
    graph.dbscan_svg(Some(dbscan.as_str()))
  } else {
    graph.dbscan_svg(None)
  }
}

#[wasm_bindgen]
pub fn pca (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "PCA");
  let pca = graph.train_pca();

  return serde_json::to_string(&pca).unwrap();
}

#[wasm_bindgen]
pub fn plot_pca (csv_content: &[u8], pca: String, trained: i32) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "PCA");
  if trained != 0 {
    graph.pca_svg(Some(pca.as_str()))
  } else {
    graph.pca_svg(None)
  }
}

#[wasm_bindgen]
pub fn gp (csv_content: &[u8]) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "gp");
  let gaussp = graph.train_gp();

  return serde_json::to_string(&gaussp).unwrap();
}

#[wasm_bindgen]
pub fn plot_gp (csv_content: &[u8], gaussp: String, trained: i32) -> String {
  let graph = prepare_graph (csv_content, 800, 400, 20, "gp");
  if trained != 0 {
    graph.gp_svg(Some(gaussp.as_str()))
  } else {
    graph.gp_svg(None)
  }
}


pub fn prepare_graph (csv_content: &[u8], width: usize, height: usize, padding: usize, title: &str) -> Graph {
  let csv_info: (Vec<f64>, (usize, usize)) = read_data(csv_content);
  let data: Vec<f64> = csv_info.0;
  let dim: (usize, usize) = csv_info.1;
  let mut xs: Vec<f64> = Vec::new();
  let mut ys: Vec<f64> = Vec::new();
  let mut tuples: Vec<(f64, f64)> = Vec::new();
  let mut labels: Vec<f64> = Vec::new();

  for i in 0..data.len() {
    if (i % 3) == 2 {
      tuples.push((data[i-2], data[i-1]));
      labels.push(data[i]);
      xs.push(data[i-2]);
      ys.push(data[i-1]);
    }
  }
  
  let width = width - padding * 2;
  let height = height - padding * 2;

  let mut graph = generate_graph(xs, ys, title, width, height, padding, dim, labels);

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


pub fn generate_graph(xs: Vec<f64>, ys: Vec<f64>, title : &str, width: usize, height: usize, 
                      padding: usize, dim: (usize, usize), labels: Vec<f64>) -> Graph {
  let mut graph = Graph::new(title.into(), "#8ff0a4".into());
  graph.size = dim.0;
  graph.attributes = dim.1;
  graph.labels = labels;
  graph.width = width;
  graph.height = height;
  graph.padding = padding;
  for i in 0..graph.size {
    graph.add_point(xs[i], ys[i]);
  }
  return graph;
} 



fn read_data(csv_content: &[u8]) -> (Vec<f64>, (usize, usize)) {
  let v : Vec<u8> = csv_content.to_vec();
  println!("INPUT length is {}", v.len());

  let mut data_reader = csv::Reader::from_reader(csv_content);
  let mut data: Vec<f64> = Vec::new();
  let mut dim: (usize, usize) = (0, 0);
  let mut read_column: bool = false;

  for record in data_reader.records() {
    dim.0 += 1;
    if !read_column {
      for field in record.unwrap().iter() {
          let value = f64::from_str(field);
          data.push(value.unwrap());
          dim.1 += 1;
      }
      read_column = true;
    } else {
      for field in record.unwrap().iter() {
        let value = f64::from_str(field);
        data.push(value.unwrap());
      }
    }
  }
  return (data, dim);
}
