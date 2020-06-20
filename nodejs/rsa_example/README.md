# The RSA encryption and decryption example

In this example, we demonstrate how to exchange complex data types between Rust and JavaScript using JSON strings. Read about [encryption and decryption in Rust and Node.js](https://www.secondstate.io/articles/encryption-and-decryption/).

## Prerequisites

If you have not done so already, follow these simple instructions to [install Rust, Node.js, SSVM, and ssvmup](https://www.secondstate.io/articles/setup-rust-nodejs/).

## The cargo config file

The [Cargo.toml](Cargo.toml) file shows the dependencies. 

* The `wasm-bindgen` crate is required for invoking these Rust functions from JavaScript.
* The `serde` and `serde_json` crates allow us to work with JSON strings to represent complex data types.
* The `rand` crate is configured to use random numbers from Node.js.

## Rust code

The [src/lib.rs](src/lib.rs) file contains three Rust functions to create a key pair, encrypt with the public key, and decrypt with the private key. The keys are passed into or returned from those functions as JSON strings.

## Build the WASM bytecode

```
$ ssvmup build
```

## Node app

The [node/app.js](node/app.js) file shows how to call the Rust functions from JavaScript to create a key pair, use the public key to encrypt a string, and then use the private key to decrypt it.

## Test

```
$ cd node
$ node app.js
generate_key_pair: 2639.036ms
encrypt_decrypt: 56.670ms
The Times 03/Jan/2009 Chancellor on brink of second bailout for banks
```
