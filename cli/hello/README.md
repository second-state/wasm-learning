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

We will use the `wasmedge` command to run the program.

```
$ wasmedge target/wasm32-wasi/debug/hello.wasm second state
hello
second
state
```

