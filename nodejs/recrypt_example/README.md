# The [Recrypt](https://github.com/IronCoreLabs/recrypt-rs) encryption and decryption example

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

$ npm install -g ssvmup # Append --unsafe-perm if permission denied
$ npm install ssvm
```

## Create new project

```
$ cargo new --lib recrypt_example
$ cd recrypt_example
```

## Change the cargo config file

The [Cargo.toml](Cargo.toml) file shows the dependencies. Note the dependency for wasm-bindgen, which is required for invoking these Rust functions from JavaScript. The dependency for serde and serde-json allows us to work with JSON strings to represent complex data types.

## Write Rust code

The [src/lib.rs](src/lib.rs) file contains three Rust functions to create a key pair, encrypt with the public key, and decrypt with the private key. The keys are passed into or returned from those functions as JSON strings.

## Build the WASM bytecode

```
$ ssvmup build
```

## Create a new Node folder

```
$ mkdir node
$ cp pkg/recrypt_example_lib_bg.wasm node/
$ cp pkg/recrypt_example_lib.js node/
$ cd node
```

## Create a node file

The [node/app.js](node/app.js) file shows how to call the Rust functions from JavaScript.

## Test

```
node app.js
```
