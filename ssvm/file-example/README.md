# WASI standalone application demo

In this example, we demonstrate how to access system resources from the Rust main function in the SSVM.

## Prerequisites

If you have not done so already, follow these simple instructions to [install Rust, Node.js, SSVM, and ssvmup](https://www.secondstate.io/articles/setup-rust-nodejs/).

## Build the WASM bytecode

```
$ cargo build --release --target wasm32-wasi
```

## Node.js app

```
node app.js
```
