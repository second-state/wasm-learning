# The functions example

In this example, we demonstrate how to call Rust functions in SSVM from JavaScript.

## Set up

```
$ sudo apt-get update
$ sudo apt-get -y upgrade
$ sudo apt install build-essential curl wget git vim libboost-all-dev

$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env

$ curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | bash
$ [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
$ [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"

$ nvm install v10.19.0
$ nvm use v10.19.0

$ npm install -g ssvmup # Append --unsafe-perm if permission denied
$ npm install ssvm
```

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
$ ssvmup build
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
