# The hello example

In this example, we setup a `string` hello world WASM demo and demonstrate interaction with the browser's JS host. Since the `string` type is NOT natively supported by WASM, we use the `wasm-pack` and `wasm-bindgen` utilities to generate corresponding call stub objects in JS.

## Set up

```
$ sudo apt-get update
$ sudo apt-get -y upgrade
$ sudo apt install build-essential

$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env

$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
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
wasm-bindgen = "0.2.50"
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
$ wasm-pack build --target nodejs
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
