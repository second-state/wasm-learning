# The YOLOv4 TensorFlow Lite example

Run YOLO model as functions.

[Live Demo](https://second-state.github.io/wasm-learning/faas/yolo-tflite/html/index.html)

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## TensorFlow and Python prerequisites

### Install these dependencies
pip3 install opencv-python==4.1.1.6
pip3 install lxml
pip3 install tqdm
pip3 install tensorflow==2.3.0rc0
pip3 install absl-py
pip3 install easydict
pip3 install matplotlib
pip3 install pillow

### Clone this repo

https://github.com/hunglc007/tensorflow-yolov4-tflite.git

### Download the weights in to the `data` directory

https://drive.google.com/open?id=1cewMfusmPjYWbrnuJRuKhPMwRe_b9PaT


Run the following command to convert weights to tensorflow

```
python3 save_model.py --weights ./data/yolov4.weights --output ./checkpoints/yolov4-416 --input_size 416 --model yolov4
```

We can now save the tf model for tflite converting

```
python3 save_model.py --weights ./data/yolov4.weights --output ./checkpoints/yolov4-416 --input_size 416 --model yolov4 --framework tflite
```

The above command will, amongst other things, create a `saved_model.pb` file in the `checkpoints/yolov4-416` directory

Now we will convert the `.pb` file to `.tflite` file

```
python3 convert_tflite.py --weights ./checkpoints/yolov4-416 --output ./checkpoints/yolov4-416.tflite
```

The above command creates a `yolov4-416.tflite` file in the checkpoints directory.

## Build the WASM bytecode

```bash
rustup target add wasm32-wasi
```

```bash
rustwasmc build --enable-aot
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

# Option 1
## Wasm

We set up node to execute `.wasm` file via WasmEdge like this

```javascript
// Import file system library
const fs = require('fs');

// Create ssvm instance
const ssvm = require("ssvm-extensions");


// Use this first time (initial call)
const path = "/media/nvme/yolo/wasm-learning/faas/yolo-tflite/pkg/yolo_tflite_lib_bg.wasm";
vm = new ssvm.VM(path, { args:process.argv, env:process.env, preopens:{"/": "/tmp"} });

// Open image
var img_src = fs.readFileSync("image.png");

// Run function
var return_value = vm.RunUint8Array("infer", img_src);

```

# Option 2
## AOT

We set up node to create an AOT executable which we execute via WasmEdge like this

```javascript
// Import file system library
const fs = require('fs');

// Create ssvm instance
const ssvm = require("ssvm-extensions");


// Use this first time (initial call)
const path = "/media/nvme/yolo/wasm-learning/faas/yolo-tflite/pkg/yolo_tflite_lib_bg.wasm";
vm = new ssvm.VM(path, { args:process.argv, env:process.env, preopens:{"/": "/tmp"} });

// AOT path
aot_path = "/media/nvme/node_rpc/aot_file"

// If you want to, please go ahead and make an aot file
vm.Compile(aot_path);

// Use this after the first time (subsequent calls)
var vm_aot = new ssvm.VM(aot_path, { args:process.argv, env:process.env, preopens:{"/": "/tmp"} });

// Open image
var img_src = fs.readFileSync("image.png");

// Run function
var return_value = vm_aot.RunUint8Array("infer", img_src);

```

Run the local test on Node.js.

```
$ cd test
$ node test.js
Drawing box: 30 results ...
Face Detection: 888.961ms
```


