# The RSA encryption and decryption example

In this example, we demonstrate how to exchange complex data types between Rust and JavaScript using JSON strings. We will also show the performance advantages of Rust over JavaScript.

## Set up

```
$ sudo apt-get update
$ sudo apt-get -y upgrade
$ sudo apt install build-essential curl wget git vim libboost-all-dev

$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env

$ curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | bash
$ export NVM_DIR="$HOME/.nvm"
$ [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
$ [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"

$ nvm install v10.19.0
$ nvm use v10.19.0

$ npm install -g wasm-pack
```

## Create new project

```
$ cargo new --lib rsa_example
$ cd rsa_example
```

## Change the cargo config file

The [Cargo.toml](Cargo.toml) file shows the dependencies. 

* The `wasm-bindgen` crate is required for invoking these Rust functions from JavaScript.
* The `serde` and `serde_json` crates allow us to work with JSON strings to represent complex data types.
* The `rand` crate is configured to use random numbers from Node.js.

## Write Rust code

The [src/lib.rs](src/lib.rs) file contains three Rust functions to create a key pair, encrypt with the public key, and decrypt with the private key. The keys are passed into or returned from those functions as JSON strings.

## Build the WASM bytecode

```
$ wasm-pack build --target nodejs
```

## Create a node file

The [node/app.js](node/app.js) file shows how to call the Rust functions from JavaScript to create a key pair, use the public key to encrypt a string, and then use the private key to decrypt it.

## Test

```
$ cd node
$ node app.js
```

## Performance

To compare performance with a pure JS implementation of the RSA algorithms. You can try this.

```
$ node app_jsencrypt.js
```
