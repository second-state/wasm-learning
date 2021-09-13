# Example for image classification using the WasmEdge Tensorflow extension API

## Prerequisite

You need to install [WasmEdge](https://github.com/WasmEdge/WasmEdge/blob/master/docs/install.md) and [Rust](https://www.rust-lang.org/tools/install).

## Build

```
$ cargo build --target wasm32-wasi --release
```

## Run

```
$ wasmedge-tensorflow-lite target/wasm32-wasi/release/classify.wasm < grace_hopper.jpg
It is very likely a <a href='https://www.google.com/search?q=military uniform'>military uniform</a> in the picture
```

## Make it run faster

```
$ wasmedgec-tensorflow target/wasm32-wasi/release/classify.wasm classify.so
$ wasmedge-tensorflow-lite classify.so < grace_hopper.jpg
It is very likely a <a href='https://www.google.com/search?q=military uniform'>military uniform</a> in the picture
```

