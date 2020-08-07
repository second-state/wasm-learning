const express = require('express');
const fileUpload = require('express-fileupload');
const { fit_draw } = require('../pkg/ml_lib.js');

const app = express();
const port = 8080;
app.use(express.static(__dirname + '/public'));
app.use(fileUpload());

app.get('/', (req, res) => res.redirect("/index.html"));

app.post('/draw', function (req, res) {
  if (!req.files || Object.keys(req.files).length === 0) {
    return res.status(400).send('No files were uploaded.');
  }
  console.log ("Received " + req.files.csv_file.name + " with size: " + req.files.csv_file.size);
  console.log ("Received " + req.body.num);
  console.log ("Received " + req.body.title);

  let csv_file = req.files.csv_file;
  console.time(csv_file.name);
  var svg = fit_draw(csv_file.data, parseInt(req.body.num), 800, 400, 50, req.body.title);
  console.timeEnd(csv_file.name);
  res.send(svg)
})

app.listen(port, () => console.log(`Listening at http://localhost:${port}`))

