const { lin_reg_train, lin_reg_svg, log_reg_train, log_reg_svg, glm_train, glm_svg, kmeans_train, kmeans_svg } = require('../pkg/algos_lib.js');

const fs = require('fs');
var iris_csv = fs.readFileSync("iris.data.csv");
var iris_values_csv = fs.readFileSync("iris_values.data.csv");
var model;

console.time('algos');
model = lin_reg_train(iris_csv);
fs.writeFileSync("lin_reg.svg", lin_reg_svg(iris_csv, model));
console.timeLog('algos', "lin_reg");

model = log_reg_train(iris_values_csv);
fs.writeFileSync("log_reg.svg", log_reg_svg(iris_csv, model));
console.timeLog('algos', "log_reg");

model = glm_train(iris_values_csv);
fs.writeFileSync("glm.svg", glm_svg(iris_csv, model));
console.timeLog('algos', "glm");

model = kmeans_train(iris_csv, 2);
fs.writeFileSync("kmeans.svg", kmeans_svg(iris_csv, model));
console.timeLog('algos', "kmeans");

console.timeEnd('algos');
