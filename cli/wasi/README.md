# WASI standalone app

In this example, we demonstrate how to run a standalone WASM application from the command line.

## Prerequisites

If you have not done so already, follow these simple instructions to [install Rust, Node.js, WasmEdge, and rustwasmc](https://www.secondstate.io/articles/setup-rust-nodejs/).

## Build the WASM bytecode

```
$ rustwasmc build --enable-aot
```

## Run the application from command line

We will use the WasmEdge command to run the program.

```
$ wasmedge --dir .:. pkg/wasi_example_main.wasm arg1 arg2
```
