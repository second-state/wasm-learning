# The YOLOv4 TensorFlow Lite example

Run YOLO model as functions.

[Live Demo](https://second-state.github.io/wasm-learning/faas/yolov5-tflite/html/index.html)

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## TensorFlow and Python prerequisites

### Install these dependencies
sudo apt install python3-pip
pip3 install numpy
pip3 install tensorflow
pip3 install torch
pip3 install pandas
pip3 install Pillow
sudo apt-get update
sudo apt-get install python3-opencv
pip3 install opencv-python
pip3 install tqdm
pip3 install torchvision
pip3 install matplotlib
pip3 install seaborn

### Clone this repo

Obtain the forked code (which will hopefully be merged into the official YOLOv5 repo via the aforementioned PR in good time)
```
git clone https://github.com/zldrobit/yolov5.git
cd yolov5
```

Next, we export TensorFlow models (GraphDef and saved model)
```
python models/tf.py - weights weights/yolov5s.pt - cfg models/yolov5s.yaml - img 320
```

We now fetch, unzip and position the COCO train dataset (which is very large)
```
wget http://images.cocodataset.org/zips/train2017.zip
unzip train2017.zip
mv train2017 data/
```

With the training dataset in place, we can now export (create a .tflite file) using the following command
```
python3 models/tf.py --weights weights/yolov5s.pt --cfg models/yolov5s.yaml --tfl-int8 --source data/train2017 --ncalib 100

```

The above command will generate a new file called `yolov5s-int8.tflite` in the weights directory (weights/yolov5s-int8.tflite)

## Build the WASM bytecode

```bash
rustup target add wasm32-wasi
```

```bash
rustwasmc build
```

## Compile to WASM bytecode

```
rustwasmc build
```

## Perform AOT optimisations

```javascript
// Import file system library
const fs = require('fs');
// Create wasmedge instance
const wasmedge = require("wasmedge-extensions");
// Load the .wasm file
const path = "/media/nvme/yolov5_wasm/yolo_tflite_lib_bg.wasm";
// Create a WebAssembly VM Instance
var vm = new wasmedge.VM(path, { EnableAOT:true, rgs:process.argv, env:process.env, preopens:{"/": "/tmp"} });
// Create a file path for ahead of time compiled (optimized) binary
aot_path = "/media/nvme/aot_file.so";
// Make an AOT optimized executable file
vm.Compile(aot_path);
// Create new VM instance using AOT (as apposed to wasm interpreted)
var vm_aot = new wasmedge.VM(aot_path, { EnableAOT:true, rgs:process.argv, env:process.env, preopens:{"/": "/tmp"} });
```

## Run via NodeJS
```javascript
// Open the image which we will perform object detection on
var img_src = fs.readFileSync("image.png");
// Run by passing in the image as a byte array
var return_value = vm_aot.RunUint8Array("infer", img_src);
```
