# WASI

In this example, we demonstrate how to access system resources from Rust functions in WasmEdge. Read about [Access operating system resources from WebAssembly](https://www.secondstate.io/articles/wasi-access-system-resources/)

## Prerequisites

If you have not done so already, follow these simple instructions to [install Rust, Node.js, WasmEdge, and rustwasmc](https://www.secondstate.io/articles/setup-rust-nodejs/).

## Build the WASM bytecode

```
$ rustwasmc build
```

## Node.js app

```
node app.js
```
