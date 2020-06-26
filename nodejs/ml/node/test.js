const { fit_draw } = require('../pkg/ml_lib.js');

const fs = require('fs');

console.log("iris cluster centers");
var iris_csv = fs.readFileSync("iris.data.csv");
var svg = fit_draw(iris_csv, 3, 800, 400, 50, "Demo");
console.log(svg);
