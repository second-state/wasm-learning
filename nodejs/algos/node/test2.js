const { lin_reg_train, lin_reg_svg, log_reg_train, log_reg_svg, glm_train, glm_svg, kmeans_train, kmeans_svg, svm_train, svm_svg, gmm_train, gmm_svg, dbscan_train, dbscan_svg, pca_train, pca_svg } = require('../pkg/algos_lib.js');

const fs = require('fs');
var birch3_csv = fs.readFileSync("public/birch3.csv");
var model;

model = kmeans_train(birch3_csv, 15);
fs.writeFileSync("kmeans.svg", kmeans_svg(birch3_csv, model));
console.timeLog('algos', "kmeans");

model = gmm_train(birch3_csv, 15);
fs.writeFileSync("gmm.svg", gmm_svg(birch3_csv, model));
console.timeLog('algos', "gmm");

console.timeEnd('algos');
