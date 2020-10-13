# The functions example

In this example, we demonstrate how to call Rust functions in SSVM from JavaScript. Read about [Getting started with Rust functions in Node.js](https://www.secondstate.io/articles/getting-started-with-rust-function/).

## Prerequisites

If you have not done so already, follow these simple instructions to [install Rust, Node.js, SSVM, and ssvmup](https://www.secondstate.io/articles/setup-rust-nodejs/).


## Create new project

```
$ cargo new --lib functions
$ cd functions
```

## Change the cargo config file

The [Cargo.toml](Cargo.toml) file shows the dependencies. Note the dependency for wasm-bindgen, which is required for invoking these Rust functions from JavaScript. The dependency for serde and serde-json allows us to work with JSON strings to represent complex data types.

## Write Rust code

The supported data types are:

* Call parameters can be any combo of i32, String, &str, Vec<u8>, and &[u8]
* Return value can be i32 or String or Vec<u8>
* For complex data types, such as structs, you could return a JSON string

The [src/lib.rs](src/lib.rs) file contains a few Rust functions that showcase different combinations of input / output parameters, including using JSON strings to serialize and deserialize complex data types.

## Build the WASM bytecode

```
$ ssvmup build --enable-aot
```

## Node.js app

The [node/app.js](node/app.js) file shows how to call the Rust functions, running inside the Second State VM (SSVM), from JavaScript inside Node.js.

## Test

```
node app.js
```

## Result
```
hello SSVM
...
246
Uint8Array [87, ... 203]
Uint8Array [126, ... 27]
```
