# The json I/O example

In this example, we demonstrate how to use JSON to call Rust functions in SSVM from JavaScript.

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
$ cargo new --lib json_io
$ cd json_io
```

## Change the cargo config file

The [Cargo.toml](Cargo.toml) file shows the dependencies. Note the dependency for wasm-bindgen, which is required for invoking these Rust functions from JavaScript. The dependency for serde and serde-json allows us to work with JSON strings to represent complex data types.

## Write Rust code

The [src/lib.rs](src/lib.rs) file contains a few Rust functions that showcase using JSON for input / output parameters. Multiple parameters and return values are represented in tuples.

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
62.831802
50.0
[0.5,-3.0,true]
{"points":[{"x":1.5,"y":3.8},{"x":2.5,"y":5.8}],"valid":true,"length":2.2360682,"desc":"A thin red line"}
```
