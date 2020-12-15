# WASI standalone app

In this example, we demonstrate how to run a standalone WASM application from the command line.

## Prerequisites

If you have not done so already, follow these simple instructions to [install Rust, Node.js, SSVM, and ssvmup](https://www.secondstate.io/articles/setup-rust-nodejs/).

## Build the WASM bytecode

```
$ ssvmup build --enable-aot
```

## Run the application from command line

We will use the SSVM command to run the program.

```
$ ssvm pkg/wasi_example_main.wasm arg1 arg2
```

The `ssvmr` command can run the AOT-compiled `.so` program.

```
$ ssvmr pkg/wasi_example_main.so arg1 arg2
```

