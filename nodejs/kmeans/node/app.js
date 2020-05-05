const { fit } = require('../pkg/kmeans_lib.js');

var means = JSON.parse(fit(JSON.stringify(["birch3.data.csv", 2, 100])));
console.log("birch3 cluster centers");
console.log(means);

var means = JSON.parse(fit(JSON.stringify(["iris.data.csv", 2, 3])));
console.log("iris cluster centers");
console.log(means);

var means = JSON.parse(fit(JSON.stringify(["s1.data.csv", 2, 15])));
console.log("s1 cluster centers");
console.log(means);

var means = JSON.parse(fit(JSON.stringify(["dim128.data.csv", 128, 16])));
console.log("dim128 cluster centers");
console.log(means);
