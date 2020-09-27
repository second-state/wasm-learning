const { infer } = require('../pkg/mtcnn_service_lib.js');
const fs = require('fs');

var img_src = fs.readFileSync("solvay.jpg");
console.log("Done reading file");
var img_res = infer(img_src);
console.log("Done inference");
fs.writeFileSync("res.png", img_res);
console.log("Done writing file");
