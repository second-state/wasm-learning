# WASI wasmedge-tensorflow app

In this example, we demonstrate how to run a wasmedge_tensorflow_interface WASM application from the command line.

## Prerequisites

[Install Rust](https://www.rust-lang.org/tools/install)

Install WasmEdge by downloading and extracting the `wasmedge-tensorflow` binary executable file from [project release package](https://github.com/second-state/WasmEdge-tensorflow/releases).

## Build the WASM bytecode

```
$ cargo build --target wasm32-wasi --release
```

## Run the application from command line

We will use the `wasmedge-tensorflow` command to run the program.

```
wasmedge-tensorflow --dir .:. target/wasm32-wasi/release/classify_yml.wasm config/bird.yml img/bird.png

Haliaeetus leucocephalus,0.6432397
Caracara cheriway,0.08298906
Parabuteo unicinctus,0.040016767
Phalacrocorax carbo,0.01358868
```

