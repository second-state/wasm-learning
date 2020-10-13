# WASI

In this example, we demonstrate how to access system resources from Rust functions in the SSVM. Read about [Access operating system resources from WebAssembly](https://www.secondstate.io/articles/wasi-access-system-resources/)

## Prerequisites

If you have not done so already, follow these simple instructions to [install Rust, Node.js, SSVM, and ssvmup](https://www.secondstate.io/articles/setup-rust-nodejs/).

## Build the WASM bytecode

```
$ ssvmup build --enable-aot
```

## Node.js app

```
node app.js
```
