# The TensorFlow Lite Single Shot Detector (SSD) example

Run SSD model as functions.

[Live Demo](https://second-state.github.io/wasm-learning/faas/tflite_ssd/html/index.html)

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Getting the SSD model trained

There is a process to create and train a model. You can find the full instructions here on [the official TensorFlow site](https://www.tensorflow.org/lite/tutorials/model_maker_object_detection)

Once you have a model, please add the path to your model (inside this repository's lib.rs file)

```
let model_data: &[u8] = include_bytes!("/path/to/your/model.tflite");
```

## Build the WASM bytecode

```bash
rustup target add wasm32-wasi
```

```bash
rustwasmc build
```

## Run the WASM bytecode

You must have Node.js and NPM installed to proceed.

Install TensorFlow Lite

```
wget https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-cpu-linux-x86_64-2.3.0.tar.gz
sudo tar -C /usr/local -xzf libtensorflow-cpu-linux-x86_64-2.3.0.tar.gz
sudo ldconfig
```

Install dependencies

```bash
sudo apt-get update
sudo apt-get -y upgrade
sudo apt install build-essential curl wget git vim libboost-all-dev llvm-dev liblld-10-dev
```

In addition you will need a Nodejs addon, which you can [install via these instructions](https://github.com/second-state/wasm-joey/blob/master/documentation/installation.md#ssvm-nodejs-add-on)


## Wasm

We set up node to execute `.wasm` file via WasmEdge like this, notice that we create an ahead of time compiled executable also.

```javascript
// Import file system library
const fs = require('fs');

// Create ssvm instance
const ssvm = require("ssvm-extensions");


// Use this first time (initial call)
const path = "/media/nvme/yolo/wasm-learning/faas/yolov4-tflite/pkg/yolo_tflite_lib_bg.wasm";
vm = new ssvm.VM(path, { args:process.argv, env:process.env, preopens:{"/": "/tmp"} });

// AOT path
aot_path = "/media/nvme/aot_file.so"

// If you want to, please go ahead and make an aot file
vm.Compile(aot_path);

// Use this after the first time (subsequent calls)
var vm_aot = new ssvm.VM(aot_path, { EnableAOT:true, rgs:process.argv, env:process.env, preopens:{"/": "/tmp"} });

// Open image
var img_src = fs.readFileSync("/media/nvme/image.png");

// Run function
var return_value = vm_aot.RunUint8Array("detect", img_src);

fs.writeFileSync("res.jpg", return_value);

```
