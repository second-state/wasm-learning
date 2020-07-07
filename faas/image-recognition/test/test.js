const { infer } = require('../pkg/image_recognition_lib.js');

const fs = require('fs');
var data_img_hopper = fs.readFileSync("grace_hopper.jpg");

var result = JSON.parse( infer(data_img_hopper) );
console.log("Detected object " + result[0] + " with probability " + result[1]);
