# The k-means clustering example for machine learning

In this example, we demonstrate how to do high performance machine learning in Node.js. The computationally intensive machine learning code is written in Rust and executed in WebAssembly. The user-facing application that uses machine learning is written in JavaScript and runs in Node.js. The example does the following:

* Simulate three clusters of 2-D data points.
* Use k-means algorithm to fit a model for the simulated data points.
* Predict which cluster a newly observed point falls into.

This example is inspired by [this article](https://www.lpalmieri.com/posts/2019-12-01-taking-ml-to-production-with-rust-a-25x-speedup/), which showed that Rust produced a 25x performance gain from Python.

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

$ npm install -g wasm-pack
```

## Create new project

```
$ cargo new --lib kmeans
$ cd kmeans
```

## Change the cargo config file

The [Cargo.toml](Cargo.toml) file shows the dependencies. 

* The `wasm-bindgen` crate is required for invoking these Rust functions from JavaScript. 
* The `serde` and `serde_json` crates allow us to work with JSON strings to represent complex data types.
* The `rand` crate is configured to use random numbers from Node.js.

## Write Rust code

The [src/lib.rs](src/lib.rs) file contains Rust functions to generate simulated data, fit a model, and make predictions from the model. The data points array and model are passed into or returned from those functions as JSON strings.

## Build the WASM bytecode

```
$ wasm-pack build --target nodejs
```

## Create a node app

The [node/app.js](node/app.js) app shows how to call the Rust functions from JavaScript to generate simulated data, fit a model to the data, and predict the cluster for a new observed data point.

## Test

```
node app.js
```
