# Wasm version image flattening and calling tensorflow for face recognition

## Setup the `ssvmup`

```bash
$ npm install -g ssvmup
```

## Build

### Build wasm source

```bash
$ ssvmup build
```

The result wasm file will be at `pkg/wasm_flatten_img_bg.wasm`.

### Build ssvm

Get [ssvm](https://github.com/second-state/SSVM).

```bash
$ docker pull secondstate/ssvm
$ docker run -it --rm \
    -v <path/to/your/ssvm/source/folder>:/root/ssvm \
    secondstate/ssvm:latest
(docker)$ cd /root/ssvm
(docker)$ mkdir -p build && cd build
(docker)$ cmake -DCMAKE_BUILD_TYPE=Release -DSSVM_DISABLE_AOT_RUNTIME=On -DSTATIC_BUILD=On .. && make -j
```

### Build face_detect binary

See [face_detect](https://github.com/second-state/wasm-learning/tree/master/rust/face_detect).

```bash
# Copy face_condition binary to /usr/bin/
(docker)$ cp face_detect /usr/bin/
# Copy tensorflow shared library to /usr/lib/
(docker)$ cp libtensorflow.so.1 /usr/lib/
(docker)$ cp libtensorflow_framework.so.1 /usr/lib/
```

## Run

```bash
(docker)$ cd /root/ssvm/build/tools/ssvm
# Copy input image and wasm file to /root/ssvm/build/tools/ssvm/
(docker)$ ./ssvm --dir .:. wasm_flatten_img_bg.wasm input_path.jpg output_path.jpg
```
