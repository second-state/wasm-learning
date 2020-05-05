# The tensorflow image recognition example

In this example, we demonstrate how to do high performance AI inference in Node.js. The computationally intensive tensorflow code is written in Rust and executed in WebAssembly. The user-facing application that uses image recognition is written in JavaScript and runs in Node.js.

## Set up

```
$ sudo apt-get update
$ sudo apt-get -y upgrade
$ sudo apt install build-essential curl wget git vim libboost-all-dev

$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env

$ curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | bash
$ [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
$ [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"

$ nvm install v10.19.0
$ nvm use v10.19.0

$ npm install -g wasm-pack
```

## Create new project

```
$ cargo new --lib tensorflow
$ cd tensorflow
```

## Change the cargo config file

The [Cargo.toml](Cargo.toml) file shows the dependencies.

* The `wasm-bindgen` crate is required for invoking Rust functions from JavaScript. 
* The `serde` and `serde_json` crates allow us to work with JSON strings to represent complex data types. 
* The `nodejs-helper` crate allows the Rust function to access console, file system, database, and network.
* The `images` crate only enables features that are compatible with WebAssembly.

## Write Rust code

The [src/lib.rs](src/lib.rs) file contains Rust functions to read the tensorflow model from a file, read and resize an image, and then run the model against the image to recognize the image subject. The result is returned as a JSON array containing the the ImageNet category ID for the recognized object, and the confidence level for this prediction. [Learn more](https://github.com/tensorflow/models/tree/master/research/slim/nets/mobilenet) about this example.

## Build the WASM bytecode

```
$ wasm-pack build --target nodejs
```

## Create a node app

The [node/app.js](node/app.js) app shows how to call the Rust functions from JavaScript. It uses a [pre-trained tensorflow model](https://storage.googleapis.com/mobilenet_v2/checkpoints/mobilenet_v2_1.4_224.tgz) to recognize two images.

## Test

```
$ node app.js
```

The first task is to recognize an image of computer scientist Grace Hopper. It takes 0.9s to recognize this image.

```
Model: "mobilenet_v2_1.4_224_frozen.pb"
Image: "grace_hopper.jpg"
Inference: 131.783ms Model loaded
Inference: 367.625ms Plan loaded
Inference: 391.095ms Image loaded
Inference: 427.137ms Image resized
Inference: 1322.184ms Model applied
Inference: 1322.637ms
Detected object id 654 with probability 0.3256046
```

Category ID `654` can be found in the [imagenet_slim_labels.txt](imagenet_slim_labels.txt). Line `654`.

```
654 military uniform
```

The second task is to recognize an image of a cat. It takes 0.8s to recognize this image.

```
Model: "mobilenet_v2_1.4_224_frozen.pb"
Image: "cat.png"
Inference: 86.587ms Model loaded
Inference: 314.308ms Plan loaded
Inference: 842.836ms Image loaded
Inference: 1166.115ms Image resized
Inference: 2014.337ms Model applied
Inference: 2014.602ms
Detected object id 284 with probability 0.27039126
```

Category ID `284` can be found in the [imagenet_slim_labels.txt](imagenet_slim_labels.txt). Line `284`.

```
284 tiger cat
```

