# The SVG example

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
$ npm install ssvm
```

## Build the WASM bytecode

```
$ ssvmup build
```

## Run the Node.js app

```
$ cd node
$ node server.js
```

## User testing

Point your browser to http://localhost:8080/ and hit the Draw button.

## Test

```
$ node node/test.js
<?xml version="1.0" standalone="no"?>
<svg
  ... ...
</svg>
```
