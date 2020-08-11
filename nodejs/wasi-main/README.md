# WASI standalone app

In this example, we demonstrate how to run a standalone WASM application from the command line.

## Prerequisites

If you have not done so already, follow these simple instructions to [install Rust, Node.js, SSVM, and ssvmup](https://www.secondstate.io/articles/setup-rust-nodejs/).

## Build the WASM bytecode

```
$ ssvmup build
```

## Run the application from command line

We will use the Node.js command line to bootstrap the SSVM.

```
$ node pkg/wasi_example_main.js arg1 arg2
```
