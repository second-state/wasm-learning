# The SVG example

In this example, we demonstrate how to use JSON to call Rust functions in SSVM from JavaScript.

## Prerequisites

If you have not done so already, follow these simple instructions to [install Rust, Node.js, SSVM, and ssvmup](https://www.secondstate.io/articles/setup-rust-nodejs/).

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
