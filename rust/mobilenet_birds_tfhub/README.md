# SSVM-Tensorflow Example for Running Mobilenet_v2 Model

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

### Get ssvm-tensorflow

### Option 1. Build ssvm-tensorflow

Get [ssvm-tensorflow](https://github.com/second-state/ssvm-tensorflow).

```bash
$ docker pull secondstate/ssvm
$ docker run -it --rm \
    -v <path/to/your/ssvm-tensorflow/source/folder>:/root/ssvm-tensorflow \
    secondstate/ssvm:latest
(docker)$ cd /root/ssvm-tensorflow
(docker)$ mkdir -p build && cd build
# Build ssvm-tensorflow
(docker)$ cmake -DCMAKE_BUILD_TYPE=Release .. && make -j
```

### Option 2. Get ssvm-tensorflow release version

```bash
wget https://github.com/second-state/ssvm-tensorflow/releases/download/0.1.0/ssvm-tensorflow-0.1.0-linux-x64.tar.gz
tar -zxvf ssvm-tensorflow-0.1.0-linux-x64.tar.gz
./download_dependencies  # Download the required shared libraries and make symbolic links.
```

## Run

Interpreter mode:

```bash
# Copy input image, model, and wasm file to the current directory.
LD_LIBRARY_PATH=. ./ssvm-tensorflow --dir .:. mobilenet_v2.wasm frozen.pb PurpleGallinule.jpg
```

AOT mode:

```bash
# Copy input image, model, and wasm file to the current directory.
./ssvmc-tensorflow mobilenet_v2.wasm mobilenet_v2.wasm.so
(LD_LIBRARY_PATH=. ./ssvm-tensorflow --dir .:. mobilenet_v2.wasm.so frozen.pb PurpleGallinule.jpg
```

The output will be:
```bash
576 : 0.8523263
```

Which is index 576 (0-based index) with rate 0.8523263
