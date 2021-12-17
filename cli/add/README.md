# WASI standalone app

In this example, we demonstrate how to run a standalone WASM application from the command line.

## Prerequisites

[Install Rust](https://www.rust-lang.org/tools/install)

Install WasmEdge by downloading and extracting the `wasmedge` binary executable file from [project release package](https://github.com/WasmEdge/WasmEdge/releases/).

## Build the WASM bytecode

```
$ cargo build --target wasm32-wasi
```

## Run the application from command line

We will use `wasmedge` in reactor mode to run the program.

```
$ wasmedge --reactor target/wasm32-wasi/debug/add.wasm add 2 2
4
```

