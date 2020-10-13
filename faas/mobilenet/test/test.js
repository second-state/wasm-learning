const { infer } = require('../pkg/mobilenet_service_lib.js');
const fs = require('fs');

console.time("Image Classification");
var img_src = fs.readFileSync("grace_hopper.jpg");
console.timeLog("Image Classification");
console.log("Result is: ", infer(img_src));
console.timeEnd("Image Classification");
