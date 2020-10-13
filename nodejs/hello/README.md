# The hello example

In this example, we demonstrate how to call a Rust function from a nodejs host through the SSVM. Read about [Getting started with Rust functions in Node.js](https://www.secondstate.io/articles/getting-started-with-rust-function/).

## Prerequisites

If you have not done so already, follow these simple instructions to [install Rust, Node.js, SSVM, and ssvmup](https://www.secondstate.io/articles/setup-rust-nodejs/).


## Create new project

```
$ cargo new --lib hello
$ cd hello
```

## Change the cargo config file

Add the following to the [Cargo.toml](Cargo.toml) file.

```
[lib]
name = "hello_lib"
path = "src/lib.rs"
crate-type =["cdylib"]

[dependencies]
wasm-bindgen = "0.2.59"
```

## Write Rust code

Below is the entire content of the [src/lib.rs](src/lib.rs) file.

```
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let r = String::from("hello ");
  return r + s;
}
```

## Build the WASM bytecode

```
$ ssvmup build --enable-aot
```

## Node.js app

Below is the content of the [node/app.js](node/app.js) file.

```
const { say } = require('../pkg/hello_lib.js');

const http = require('http');
const url = require('url');
const hostname = '127.0.0.1';
const port = 8080;

const server = http.createServer((req, res) => {
  const queryObject = url.parse(req.url,true).query;
  res.statusCode = 200;
  res.setHeader('Content-Type', 'text/plain');
  res.end(say(queryObject['name']));
});

server.listen(port, hostname, () => {});
```

## Test

```
node app.js
```

## Result
```
curl http://127.0.0.1:8080/?name=World!
hello World!
```
