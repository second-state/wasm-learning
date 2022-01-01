// Import file system library
const fs = require('fs');

// Create wasmedge instance
const wasmedge = require("wasmedge-extensions");

// Load the wasm binary
const path = "../pkg/mtcnn_service_lib_bg.wasm";
vm = new wasmedge.VM(path, { args:process.argv, env:process.env, preopens:{"/": "/tmp"} });

// Define an AOT path
aot_path = "mtcnn_service_lib_bg.so";

// Create an AOT binary
vm.Compile(aot_path);

// Instantiate VM using AOT mode
var vm_aot = new wasmedge.VM(aot_path, { EnableAOT:true, rgs:process.argv, env:process.env, preopens:{"/": "/tmp"} });

// Open image
var img_src = fs.readFileSync("../test/solvay.jpg");

// Run function by passing in the image and calling the infer function
var return_value = vm_aot.RunUint8Array("infer", img_src);

// Write image to file
fs.writeFileSync("res.jpg", return_value);
