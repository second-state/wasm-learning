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

### Get ssvm-tensorflow

### Option 1. Build ssvm-tensorflow

Get [ssvm-tensorflow-tools](https://github.com/second-state/ssvm-tensorflow-tools).

```bash
$ docker pull secondstate/ssvm
$ docker run -it --rm \
    -v <path/to/your/ssvm-tensorflow/source/folder>:/root/ssvm-tensorflow-tools \
    secondstate/ssvm:latest
(docker)$ cd /root/ssvm-tensorflow-tools
(docker)$ mkdir -p build && cd build
# Build ssvm-tensorflow-tools
(docker)$ cmake -DCMAKE_BUILD_TYPE=Release .. && make -j
```

### Option 2. Get ssvm-tensorflow-tools release version

```bash
wget https://github.com/second-state/ssvm-tensorflow-tools/releases/download/0.8.0-rc1/ssvm-tensorflow-tools-0.8.0-rc1-manylinux2014_x86_64.tar.gz
tar -zxvf ssvm-tensorflow-tools-0.8.0-rc1-manylinux2014_x86_64.tar.gz
./download_dependencies_all.sh  # Download the required shared libraries and make symbolic links.
```

## Run

Interpreter mode:

```bash
# Copy input image, model, and wasm file to the current directory.
LD_LIBRARY_PATH=. ./ssvm-tensorflow --dir .:. birds_v1.wasm lite-model_aiy_vision_classifier_birds_V1_3.tflite bird.jpg
```

AOT mode:

```bash
# Copy input image, model, and wasm file to the current directory.
./ssvmc-tensorflow birds_v1.wasm birds_v1.wasm.so
LD_LIBRARY_PATH=. ./ssvm-tensorflow --dir .:. birds_v1.wasm.so lite-model_aiy_vision_classifier_birds_V1_3.tflite bird.jpg
```

The output will be:
```bash
166 : 0.87058824
```

Which is index 166 (0-based index) with rate 0.87058824.
The index 166 of label table (which is line 167 in `aiy_birds_V1_labels.txt`) is `Aix galericulata`.
