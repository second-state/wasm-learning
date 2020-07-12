const express = require('express');
const fileUpload = require('express-fileupload');
const { lin_reg, log_reg, glm, kmeans, nnet, svm, gmm, nb, dbscan } = require('../pkg/algos_lib.js');

const app = express();
const port = 8080;
app.use(express.static('public'));
app.use(fileUpload());

app.get('/', (req, res) => res.redirect("/index.html"));

app.post('/draw', function (req, res) {
  if (!req.files || Object.keys(req.files).length === 0) {
    return res.status(400).send('No files were uploaded.');
  }
  console.log ("Received " + req.files.csv_file.name + " with size: " + req.files.csv_file.size);
  console.log ("Received " + req.body.model);

  let csv_file = req.files.csv_file;
  let model = req.body.model;
  console.time(model + "/" + csv_file.name);
  var svg = "";
  if (model == "lin_reg") {
    svg = lin_reg(csv_file.data);
  }
  if (model == "log_reg") {
    svg = log_reg(csv_file.data);
  }
  if (model == "nnet") {
    svg = nnet(csv_file.data);
  }
  if (model == "nb") {
    svg = nb(csv_file.data);
  }
  if (model == "kmeans") {
    svg = kmeans(csv_file.data, parseInt(req.body.kmeans_n));
  }
  if (model == "svm") {
    svg = svm(csv_file.data);
  }
  if (model == "glm") {
    svg = glm(csv_file.data);
  }
  if (model == "gmm") {
    svg = gmm(csv_file.data);
  }
  if (model == "dbscan") {
    svg = dbscan(csv_file.data);
  }
  console.timeEnd(model + "/" + csv_file.name);
  res.send(svg)
})

app.listen(port, () => console.log(`Listening at http://localhost:${port}`))

