# The hello example

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

The following is the content of the [Cargo.toml](functions/Cargo.toml) file.

```
[package]
name = "functions"
version = "0.1.0"
authors = ["ubuntu"]
edition = "2018"

[lib]
name = "functions_lib"
path = "src/lib.rs"
crate-type =["cdylib"]

[dependencies]
num-integer = "0.1"
sha3 = "0.8.2"
wasm-bindgen = "0.2.60"
```

## Write Rust code

The supported data types are:

* Call parameters can be any combo of i32, String, &str, Vec<u8>, and &[u8]
* Return value can be i32 or String or Vec<u8>
* For complex data types, such as structs, you could return a JSON string

Below is the entire content of the [src/lib.rs](functions/src/lib.rs) file.

```
extern crate num_integer;

use wasm_bindgen::prelude::*;
use num_integer::lcm;
use sha3::{Digest, Sha3_256, Keccak256};

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let r = String::from("hello ");
  return r + s;
}

#[wasm_bindgen]
pub fn obfusticate(s: String) -> String {
  return rot13(&s);
}

#[wasm_bindgen]
pub fn lowest_common_denominator(a: i32, b: i32) -> i32 {
  let r = lcm(a, b);
  return r;
}

#[wasm_bindgen]
pub fn sha3_digest(v: Vec<u8>) -> Vec<u8> {
  return Sha3_256::digest(&v).as_slice().to_vec();
}

#[wasm_bindgen]
pub fn keccak_digest(s: &[u8]) -> Vec<u8> {
  return Keccak256::digest(s).as_slice().to_vec();
}

pub fn rot13(text: &str) -> String {
    text.chars().map(|c| {
        match c {
            'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + 13) as char,
            'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - 13) as char,
            _ => c
        }
    }).collect()
}
```

## Build the WASM bytecode

```
$ ssvmup build
```

## Create a new Node folder

```
$ mkdir node
$ cp pkg/functions_lib_bg.wasm node/
$ cp pkg/functions_lib.js node/
$ cd node
```

## Create a node file

Below is the content of the [node/app.js](functions/node/app.js) file.

```
const { say, obfusticate, lowest_common_denominator, sha3_digest, keccak_digest } = require('./functions_lib.js');

var util = require('util');
const encoder = new util.TextEncoder();

console.log( say("SSVM") );
console.log( obfusticate("A quick brown fox jumps over the lazy dog") );
console.log( lowest_common_denominator(123, 2) );
console.log( sha3_digest(encoder.encode("This is an important message")) );
console.log( keccak_digest(encoder.encode("This is an important message")) );
```

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
