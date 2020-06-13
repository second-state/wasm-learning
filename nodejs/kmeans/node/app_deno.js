import { fit } from '../pkg/kmeans_lib.js';

console.log("birch3 cluster centers");
var birch3_csv = Deno.readFileSync("birch3.data.csv");
var means = JSON.parse( fit(birch3_csv, 2, 100) );
console.log(means);

console.log("iris cluster centers");
var iris_csv = Deno.readFileSync("iris.data.csv");
var means = JSON.parse( fit(iris_csv, 2, 3) );
console.log(means);

console.log("s1 cluster centers");
var s1_csv = Deno.readFileSync("s1.data.csv");
var means = JSON.parse( fit(s1_csv, 2, 15) );
console.log(means);

console.log("dim128 cluster centers");
var dim128_csv = Deno.readFileSync("dim128.data.csv");
var means = JSON.parse( fit(dim128_csv, 128, 16) );
console.log(means);
