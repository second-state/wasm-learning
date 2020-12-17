# SSVM-Tensorflow Example for Running MTCNN Model

## Setup the wasm32-wasi target of Cargo

```bash
$ rustup target add wasm32-wasi
```

## Build

### Build wasm source

```bash
$ cargo build --release --target=wasm32-wasi
```

The result wasm file will be at `target/wasm32-wasi/release/mtcnn.wasm`.

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
LD_LIBRARY_PATH=. ./ssvm-tensorflow --dir .:. mtcnn.wasm mtcnn.pb solvay.jpg tmp.jpg
```

AOT mode:

```bash
# Copy input image, model, and wasm file to the current directory.
./ssvmc-tensorflow mtcnn.wasm mtcnn.wasm.so
LD_LIBRARY_PATH=. ./ssvm-tensorflow --dir .:. mtcnn.wasm.so mtcnn.pb solvay.jpg tmp.jpg
```
