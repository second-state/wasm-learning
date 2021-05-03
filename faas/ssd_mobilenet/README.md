# TensorFlow Object Detection 

This tutorial demonstrates how to create a a Function as a Service (FaaS), using TensorFlow Object Detection, which is able to detect and identify multiple objects (in a single image). This tutorial uses [a pre-trained model](https://tfhub.dev/tensorflow/lite-model/ssd_mobilenet_v1/1/default/1) optimized to work with TensorFlow Lite for Object detection.

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Data preparation (optional)

We have already gone ahead and downloaed the object detection [starter model with metadata](https://www.tensorflow.org/lite/examples/object_detection/overview) and performed the data preparation (as well as made the Rust source code compatible with this new data). 

**You can skip this step** and just go ahead and run the `ssvmup build` as per the instructions below. However, it might be valuable to know how to prepare data, in the event that you have trained your own model etc. Here's how we did it ...

First we downloaded the [starter model with metadata](https://www.tensorflow.org/lite/examples/object_detection/overview) which provides a `ssd_mobilenet_v1_1_metadata_1.tflite` file. We then went ahead and unzipped that file, which provides us with 2 new files 1) the `detect.tflite` file and 2) the `labelmap.txt` file

We then went ahead and modified the `labelmap.txt` file. We simply ran the following python script called `prepare_data.py` from within the `src` directory i.e.

```
python3 prepare_data.py
```

This python script (`prepare_data.py`) opened the original Tensorflow `labelmap.txt` file and then performed some conversions to make the data compatible with SSD Object Detection;  the output from the (`prepare_data.py`) script created a new file called `labelmap_v2.txt`. You will notice that our Rust source code already points to this newly modified file (as apposed to the original file). So there is nothing for you to do.

Another area of data preparation involved interpreting the `ssd_mobilenet_v1_1_metadata_1.tflite`. This is again done programatically, this time by the official TensorFlow python tool; which can be installed using the following command

```
pip3 install tensorflow
```

Once you have this Python tool installed you can run our `interpret_data.py` file (we just hand wrote some code to interact with the `.tflite` file). 

```
python3 interpret_data.py 
```

The results (in relation to TensorFlow **input** details) will look something like this

```
Input details:
[{'name': 'normalized_input_image_tensor', 'index': 175, 'shape': array([  1, 300, 300,   3], dtype=int32), 'shape_signature': array([  1, 300, 300,   3], dtype=int32), 'dtype': <class 'numpy.uint8'>, 'quantization': (0.0078125, 128), 'quantization_parameters': {'scales': array([0.0078125], dtype=float32), 'zero_points': array([128], dtype=int32), 'quantized_dimension': 0}, 'sparsity_parameters': {}}]
```

From this output we can clearly see input specifications such as `shape` being `[  1, 300, 300,   3]` and so forth. We have already gone ahead and added these to our Rust code, so there is nothing for you to do here (unless you have trained your own model which is different to this).

The results (in relation to TensorFlow **output** details) will look something like this

```
Output details:
[{'name': 'TFLite_Detection_PostProcess', 'index': 167, 'shape': array([ 1, 10,  4], dtype=int32), 'shape_signature': array([ 1, 10,  4], dtype=int32), 'dtype': <class 'numpy.float32'>, 'quantization': (0.0, 0), 'quantization_parameters': {'scales': array([], dtype=float32), 'zero_points': array([], dtype=int32), 'quantized_dimension': 0}, 'sparsity_parameters': {}}, {'name': 'TFLite_Detection_PostProcess:1', 'index': 168, 'shape': array([ 1, 10], dtype=int32), 'shape_signature': array([ 1, 10], dtype=int32), 'dtype': <class 'numpy.float32'>, 'quantization': (0.0, 0), 'quantization_parameters': {'scales': array([], dtype=float32), 'zero_points': array([], dtype=int32), 'quantized_dimension': 0}, 'sparsity_parameters': {}}, {'name': 'TFLite_Detection_PostProcess:2', 'index': 169, 'shape': array([ 1, 10], dtype=int32), 'shape_signature': array([ 1, 10], dtype=int32), 'dtype': <class 'numpy.float32'>, 'quantization': (0.0, 0), 'quantization_parameters': {'scales': array([], dtype=float32), 'zero_points': array([], dtype=int32), 'quantized_dimension': 0}, 'sparsity_parameters': {}}, {'name': 'TFLite_Detection_PostProcess:3', 'index': 170, 'shape': array([1], dtype=int32), 'shape_signature': array([1], dtype=int32), 'dtype': <class 'numpy.float32'>, 'quantization': (0.0, 0), 'quantization_parameters': {'scales': array([], dtype=float32), 'zero_points': array([], dtype=int32), 'quantized_dimension': 0}, 'sparsity_parameters': {}}]
```

## Build the WASM bytecode

```
$ ssvmup build --enable-aot --enable-ext
```

## Create FaaS function

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: mobilenet' \
--data-binary '@pkg/mobilenet_service_lib_bg.wasm'
```

Returns

```
{"wasm_id":370,"wasm_sha256":"0x469c28daae7aba392076b4bc5ee3b43ec6d667083d8ae63207bf74b1da03fc26","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"7dxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx0c41"}
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/370' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: 7dxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx0c41' \
--data-binary '@pkg/mobilenet_service_lib_bg.wasm'
```

## Test

Make a function call via the web.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/370/detect' \
--header 'Content-Type: application/octet-stream' \
--data-binary '@test/table.jpg'
```

## Results

If we specify `session.get_output("TFLite_Detection_PostProcess")`, we get the following output.

```
[0.14933512, -0.00044572353, 0.87442255, 0.99421704, 0.012626499, 0.0016433597, 0.55680454, 1.0130334, 0.4743793, -0.001391083, 0.99335635, 0.9941391, 0.010940969, 0.16165733, 0.039153602, 0.18814152, 0.012315213, 0.10419304, 0.0423691, 0.12945074, 0.010020341, 0.37045017, 0.04007423, 0.39693436, 0.00995128, 0.21598876, 0.03861337, 0.24164882, 0.010487929, 0.32020384, 0.039606642, 0.34546155, 0.010020341, 0.4205447, 0.04007423, 0.4470289, 0.0074280985, 0.5292269, 0.03654681, 0.55928075]
```

You may recall that we had `TFLite_Detection_PostProcess` listed a few times in our output details when we ran the `interpreter.get_output_details()` function in our python `interperate_data.py` file. Well if we now specify the following 4 post process outputs in our Rust like this:

```
session.get_output("TFLite_Detection_PostProcess")
session.get_output("TFLite_Detection_PostProcess:1")
session.get_output("TFLite_Detection_PostProcess:2")
session.get_output("TFLite_Detection_PostProcess:3")
```

We get the following when we execute our Rust/Wasm at id 370.

```
[-0.010260969, 0.0011624098, 0.98118556, 0.9926089, 0.004220102, 0.53638095, 0.02292574, 0.57048565, 0.69399744, 0.466397, 0.7042571, 0.47229737, 0.004780039, 0.42075238, 0.019305974, 0.44682124, 0.0065962216, 0.5777225, 0.023609452, 0.6063846, 0.004057551, 0.36513472, 0.020028463, 0.39766008, 0.004306032, 0.30938902, 0.019779982, 0.3440371, 0.00783987, 0.63086903, 0.022365805, 0.65495706, 0.6049054, 0.51887727, 0.6189794, 0.52526283, 0.0045467815, 0.7417675, 0.019539231, 0.76585555]

[4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]

[0.34375, 0.28125, 0.25, 0.234375, 0.234375, 0.234375, 0.22265625, 0.21484375, 0.20703125, 0.19140625]

[10.0]
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
653 : 0.43212867
Finished post-processing in ... 320.089646ms
```
