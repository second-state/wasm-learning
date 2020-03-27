# The hello example

In this example, we demonstrate how to call a Rust function from a nodejs host through the SSVM.

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
$ cargo new --lib hello
$ cd hello
```

## Change the cargo config file

Add the following to the [Cargo.toml](hello/Cargo.toml) file.

```
[lib]
name = "hello_lib"
path = "src/lib.rs"
crate-type =["cdylib"]

[dependencies]
wasm-bindgen = "0.2.59"
```

## Write Rust code

Below is the entire content of the [src/lib.rs](hello/src/lib.rs) file.

```
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let r = String::from("hello ");
  return r + &s;
}
```

## Build the WASM bytecode

```
$ ssvmup build
```

## Create a new Node folder

```
$ mkdir node
$ cp pkg/hello_lib_bg.wasm node/
$ cp pkg/hello_lib.js node/
$ cp pkg/hello_lib_bg.js node
$ cd node
```

## Create a node file

Below is the content of the [node/app.js](hello/node/app.js) file.

```
const { say } = require('./hello_lib.js');
  
(async () => {
    try {
        console.log(say('World!'));
    } catch(e) {
        console.log(e, 'Error caught');
    }
})();
```

## Test

```
node app.js
```

## Result
```
hello World!
```
