const { infer } = require('../pkg/mobilenet_service_lib.js');
const fs = require('fs');

var img_src = fs.readFileSync("grace_hopper.jpg");
console.log("Done reading file");
console.time();
console.log("Result is: ", infer(img_src));
console.timeEnd();
console.log("Done inference");
