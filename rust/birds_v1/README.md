# SSVM-Tensorflow Example for Running Birds_v1 Model

## Setup the wasm32-wasi target of Cargo

```bash
$ rustup target add wasm32-wasi
```

## Build

### Build wasm source

```bash
$ cargo build --release --target=wasm32-wasi
```

The result wasm file will be at `target/wasm32-wasi/release/birds_v1.wasm`.

### Build ssvm-tensorflow

Get [ssvm-tensorflow](https://github.com/second-state/ssvm-tensorflow).

```bash
$ docker pull secondstate/ssvm
$ docker run -it --rm \
    -v <path/to/your/ssvm-tensorflow/source/folder>:/root/ssvm-tensorflow \
    secondstate/ssvm:latest
(docker)$ cd /root/ssvm-tensorflow
(docker)$ mkdir -p build && cd build
# Install the JPEG and PNG library
(docker)$ apt-get update && apt-get install -y libjpeg-dev libpng-dev
# Install the tensorflow library
(docker)$ wget https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-cpu-linux-x86_64-1.5.0.tar.gz
(docker)$ tar -C /usr/local -xzf libtensorflow-cpu-linux-x86_64-1.5.0.tar.gz
(docker)$ ldconfig
# Build ssvm-tensorflow
(docker)$ cmake -DCMAKE_BUILD_TYPE=Release .. && make -j
```

## Run

Interpreter mode:

```bash
(docker)$ cd /root/ssvm-tensorflow/build/tools
# Copy input image, model, and wasm file to /root/ssvm-tensorflow/build/tools
(docker)$ ./ssvm-tensorflow --dir .:. birds_v1.wasm lite-model_aiy_vision_classifier_birds_V1_3.tflite bird.jpg
```

AOT mode:

```bash
(docker)$ cd /root/ssvm-tensorflow/build/tools
# Copy input image, model, and wasm file to /root/ssvm-tensorflow/build/tools
(docker)$ ./ssvmc-tensorflow birds_v1.wasm birds_v1.wasm.so
(docker)$ ./ssvm-tensorflow --dir .:. birds_v1.wasm.so lite-model_aiy_vision_classifier_birds_V1_3.tflite bird.jpg
```

The output will be:
```bash
166 : 0.8627451
```

Which is index 166 (0-based index) with rate 0.8627451.
The index 166 of label table (which is line 167 in `aiy_birds_V1_labels.txt`) is `Aix galericulata`.
