const { infer } = require('../pkg/tensorflow_lib.js');

var result = JSON.parse(infer(JSON.stringify(["mobilenet_v2_1.4_224_frozen.pb", "grace_hopper.jpg", 224, 224])));
if (result[1] == 0) {
  console.log("Detected nothing");
} else {
  console.log("Detected object id " + result[1] + " with probability " + result[0]);
}

var result = JSON.parse(infer(JSON.stringify(["mobilenet_v2_1.4_224_frozen.pb", "cat.png", 224, 224])));
if (result[1] == 0) {
  console.log("Detected nothing");
} else {
  console.log("Detected object id " + result[1] + " with probability " + result[0]);
}
