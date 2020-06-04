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
$ ssvmup build
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
