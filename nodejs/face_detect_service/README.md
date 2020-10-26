# The tensorflow face detection example

## Set up

Follow the instructions to [setup Rust and WebAssembly tools in Node.js](https://www.secondstate.io/articles/setup-rust-nodejs/)

Next, install the necessary NPM packages.

```
$ npm i express-fileupload
$ npm i uuid
```

Finally, you need to install the [face_detect](https://github.com/second-state/wasm-learning/tree/master/rust/face_detect) dependency for the face detection Tensorflow model library.

```
$ cd face_detect/
$ cargo build --release
# Install the tensorflow library
$ wget https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-gpu-linux-x86_64-1.15.0.tar.gz
$ sudo tar -C /usr/local -xzf libtensorflow-gpu-linux-x86_64-1.15.0.tar.gz
$ sudo ldconfig
```

## Build the WASM bytecode

```
$ ssvmup build --enable-aot
```

## Test

```
$ cd test
$ node test.js
```

It detects faces in the `solvay.jpg` file, draws boxes around each face, and writes the output image into `result.png`.

## Web service

```
$ cd node
$ node server.js
```

Then you can access the service at `http://host:8080/index.html`. Try upload an image with faces and see the result in your browser!


