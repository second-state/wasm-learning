const express = require('express');
const fileUpload = require('express-fileupload');
const { lin_reg_train, lin_reg_svg, log_reg_train, log_reg_svg, glm_train, glm_svg, kmeans_train, kmeans_svg, svm_train, svm_svg, gmm_train, gmm_svg, dbscan_train, dbscan_svg, pca_train, pca_svg } = require('../pkg/algos_lib.js');

// Utils
var util = require('util');
const encoder = new util.TextEncoder();

const app = express();
const port = 8080;
app.use(express.static('public'));
app.use(fileUpload());

app.get('/', (req, res) => res.redirect("/index.html"));

app.post('/draw', function (req, res) {
  if (!req.files || Object.keys(req.files).length === 0) {
    return res.status(400).send('No files were uploaded.');
  }

  let csv_file = req.files.csv_file;
  let algo = req.body.algo;
  console.time(algo);
  var plot = false;
  if (req.body.fitplot == "1") {
    plot = true;
  }
  var resp_data = "";
  if (algo == "lin_reg") {
    let model = lin_reg_train(csv_file.data);
    resp_data = model;
    if (plot) {
      let svg = lin_reg_svg(csv_file.data, model);
      resp_data = svg
    }
  }
  if (algo == "log_reg") {
    let model = log_reg_train(encoder.encode(req.body.train_data));
    resp_data = model;
    if (plot) {
      let svg = log_reg_svg(csv_file.data, model);
      resp_data = svg
    }
  }
  if (algo == "glm") {
    let model = glm_train(encoder.encode(req.body.train_data));
    resp_data = model;
    if (plot) {
      let svg = glm_svg(csv_file.data, model);
      resp_data = svg
    }
  }
  if (algo == "svm") {
    let model = svm_train(encoder.encode(req.body.train_data));
    resp_data = model;
    if (plot) {
      let svg = svm_svg(csv_file.data, model);
      resp_data = svg
    }
  }
  if (algo == "kmeans") {
    let model = kmeans_train(csv_file.data, parseInt(req.body.kmeans_n));
    resp_data = model;
    if (plot) {
      let svg = kmeans_svg(csv_file.data, model);
      resp_data = svg
    }
  }
  if (algo == "gmm") {
    let model = gmm_train(csv_file.data, parseInt(req.body.gmm_n));
    resp_data = model;
    if (plot) {
      let svg = gmm_svg(csv_file.data, model);
      resp_data = svg
    }
  }
  if (algo == "dbscan") {
    let model = dbscan_train(csv_file.data);
    resp_data = model;
    if (plot) {
      let svg = dbscan_svg(csv_file.data, model);
      resp_data = svg
    }
  }
  if (algo == "pca") {
    let model = pca_train(csv_file.data);
    resp_data = model;
    if (plot) {
      let svg = pca_svg(csv_file.data, model);
      resp_data = svg
    }
  }
  console.timeEnd(algo);
  res.send(resp_data)
})

app.listen(port, () => console.log(`Listening at http://localhost:${port}`))

