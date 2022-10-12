# WasmEdge-Tensorflow Example for Running Mobilenet_v2 Model

## Setup the wasm32-wasi target of Cargo

```bash
$ rustup target add wasm32-wasi
```

## Build

### Build wasm source

```bash
$ cargo build --release --target=wasm32-wasi
```

The result wasm file will be at `target/wasm32-wasi/release/mobilenet_v2.wasm`.

### Get WasmEdge-tensorflow

### Option 1. Build WasmEdge-tensorflow

Get [WasmEdge-tensorflow-tools](https://github.com/second-state/WasmEdge-tensorflow-tools).

```bash
$ docker pull WasmEdge/WasmEdge
$ docker run -it --rm \
    -v <path/to/your/WasmEdge-tensorflow/source/folder>:/root/WasmEdge-tensorflow-tools \
    WasmEdge/WasmEdge:latest
(docker)$ cd /root/WasmEdge-tensorflow-tools
(docker)$ mkdir -p build && cd build
# Build WasmEdge-tensorflow-tools
(docker)$ cmake -DCMAKE_BUILD_TYPE=Release .. && make -j
```

### Option 2. Get WasmEdge-tensorflow-tools release version

```bash
wget https://github.com/second-state/WasmEdge-tensorflow-tools/releases/download/0.8.0/WasmEdge-tensorflow-tools-0.8.0-manylinux2014_x86_64.tar.gz
tar -zxvf WasmEdge-tensorflow-tools-0.8.0-manylinux2014_x86_64.tar.gz
./download_dependencies_all.sh  # Download the required shared libraries and make symbolic links.
```

## Run

Interpreter mode:

```bash
# Copy input image, model, and wasm file to the current directory.
LD_LIBRARY_PATH=. ./wasmedge-tensorflow --dir .:. mobilenet_v2.wasm mobilenet_v2_1.4_224_frozen.pb grace_hopper.jpg
```

AOT mode:

```bash
# Copy input image, model, and wasm file to the current directory.
./wasmedgec-tensorflow mobilenet_v2.wasm mobilenet_v2.wasm.so
LD_LIBRARY_PATH=. ./wasmedge-tensorflow --dir .:. mobilenet_v2.wasm.so mobilenet_v2_1.4_224_frozen.pb grace_hopper.jpg
```

The output will be:
```bash
653 : 0.33690106868743896
```

Which is index 653 (0-based index) with rate 0.33690106868743896.
The index 653 of label table (which is line 654 in `imagenet_slim_labels.txt`) is `military uniform`.
