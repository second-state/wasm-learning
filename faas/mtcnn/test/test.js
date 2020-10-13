const { infer } = require('../pkg/mtcnn_service_lib.js');
const fs = require('fs');

console.time("Face Detection");
var img_src = fs.readFileSync("solvay.jpg");
console.timeLog("Face Detection");
var img_res = infer(img_src);
console.timeLog("Face Detection");
fs.writeFileSync("res.png", img_res);
console.timeEnd("Face Detection");