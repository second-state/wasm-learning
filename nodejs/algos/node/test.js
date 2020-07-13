const { lin_reg, log_reg, glm, kmeans, nnet, svm, gmm, nb, dbscan } = require('../pkg/algos_lib.js');

const fs = require('fs');
var iris_csv = fs.readFileSync("iris.data.csv");

console.time('algos');
fs.writeFileSync("lin_reg.svg", lin_reg(iris_csv));
console.timeLog('algos', "lin_reg");

fs.writeFileSync("log_reg.svg", log_reg(iris_csv));
console.timeLog('algos', "log_reg");

fs.writeFileSync("glm.svg", glm(iris_csv));
console.timeLog('algos', "glm");

fs.writeFileSync("kmeans.svg", kmeans(iris_csv, 3));
console.timeLog('algos', "kmeans");

fs.writeFileSync("nnet.svg", nnet(iris_csv));
console.timeLog('algos', "nnet");

fs.writeFileSync("svm.svg", svm(iris_csv));
console.timeLog('algos', "svm");

fs.writeFileSync("gmm.svg", gmm(iris_csv, 3));
console.timeLog('algos', "gmm");

fs.writeFileSync("nb.svg", nb(iris_csv));
console.timeLog('algos', "nb");

fs.writeFileSync("dbscan.svg", dbscan(iris_csv));
console.timeLog('algos', "dbscan");

console.timeEnd('algos');
