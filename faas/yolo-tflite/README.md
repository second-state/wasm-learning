# The YOLOv4 TensorFlow Lite example

Run MTCNN tensorflow models as functions.

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

```
$ ssvmup build --enable-aot --enable-ext
```

## Create FaaS function

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: yolo4 tflite' \
--data-binary '@pkg/yolo_tflite_lib_bg.wasm'
```

Returns

```
{"wasm_id":195,"wasm_sha256":"0x469c28daae7aba392076b4bc5ee3b43ec6d667083d8ae63207bf74b1da03fc26","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"7dxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx0c41"}
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/195' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: 7dxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx0c41' \
--data-binary '@pkg/yolo_tflite_lib_bg.wasm'
```

## Test

Make a function call via the web.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/195/infer/bytes' \
--header 'Content-Type: application/octet-stream' \
--data-binary '@test/solvay.jpg' \
--output tmp.jpg
```

## Local test

You must have Node.js and NPM installed. Install SSVM extensions and dependencies.

```
$ sudo apt install -y libjpeg-dev libpng-dev
$ wget https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-cpu-linux-x86_64-2.3.0.tar.gz
$ sudo tar -C /usr/local -xzf libtensorflow-cpu-linux-x86_64-2.3.0.tar.gz
$ sudo ldconfig
$ npm i ssvm-extensions
```

Run the local test on Node.js.

```
$ cd test
$ node test.js
Drawing box: 30 results ...
Face Detection: 888.961ms
```


