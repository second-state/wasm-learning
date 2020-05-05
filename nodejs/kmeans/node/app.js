const { fit } = require('../pkg/kmeans_lib.js');

console.log("birch3 cluster centers");
var means = JSON.parse(fit(JSON.stringify(["birch3.data.csv", 2, 100])));
console.log(means);

console.log("iris cluster centers");
var means = JSON.parse(fit(JSON.stringify(["iris.data.csv", 2, 3])));
console.log(means);

console.log("s1 cluster centers");
var means = JSON.parse(fit(JSON.stringify(["s1.data.csv", 2, 15])));
console.log(means);

console.log("dim128 cluster centers");
var means = JSON.parse(fit(JSON.stringify(["dim128.data.csv", 128, 16])));
console.log(means);
