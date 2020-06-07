const { fit } = require('../pkg/kmeans_lib.js');

const fs = require('fs'); 

function buffer2bytes (buf) {
    return new Uint8Array(buf.buffer.slice(buf.byteOffset, buf.byteOffset + buf.byteLength));
}

console.log("birch3 cluster centers");
var birch3_csv = fs.readFileSync("birch3.data.csv");
var means = JSON.parse( fit(buffer2bytes(birch3_csv), 2, 100) );
console.log(means);

console.log("iris cluster centers");
var iris_csv = fs.readFileSync("iris.data.csv");
var means = JSON.parse( fit(buffer2bytes(iris_csv), 2, 3) );
console.log(means);

console.log("s1 cluster centers");
var s1_csv = fs.readFileSync("s1.data.csv");
var means = JSON.parse( fit(buffer2bytes(s1_csv), 2, 15) );
console.log(means);

console.log("dim128 cluster centers");
var dim128_csv = fs.readFileSync("dim128.data.csv");
var means = JSON.parse( fit(buffer2bytes(dim128_csv), 128, 16) );
console.log(means);
