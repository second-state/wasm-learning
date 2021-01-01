const { infer } = require('../pkg/tflite_food_lib.js');
const fs = require('fs');

console.time("Image Classification");
var img_src = fs.readFileSync("food.jpg");
console.timeLog("Image Classification");
console.log("Result is: ", infer(img_src));
console.timeEnd("Image Classification");
