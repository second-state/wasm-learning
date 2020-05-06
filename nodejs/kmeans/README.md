# The k-means clustering example for machine learning

In this example, we demonstrate how to do high performance machine learning in Node.js. The computationally intensive machine learning code is written in Rust and executed in WebAssembly. The user-facing application that uses machine learning is written in JavaScript and runs in Node.js. The example takes some [commonly used data sets](../deps/rkm#data), computes centers for clusters, and then label each data point with its cluster.

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
$ cd node
$ node app.js
```

The BIRCH3 data set is a synthetic data large set. 100000 points, 2 dimensions, 100 clusters. The program prints out the number of points in the top 3 clusters, and then the 100 points for the detected cluster centers.

```
birch3 cluster centers
Cluster #1 has 503 points
Cluster #2 has 884 points
Cluster #3 has 1845 points
{ v: 1,
  dim: [ 100, 2 ],
  data:
   [ 282047.4,
     456656.34,
     799066.94,
     ... ] }
```

The IRIS data set consists of natural data taken from measurements of different iris plants. 150 points, 2 dimensions, 3 clusters. The program prints out the number of points in each of the clusters, and then the 3 points for the detected cluster centers.

```
iris cluster centers
Cluster #1 has 53 points
Cluster #2 has 48 points
Cluster #3 has 48 points
{ v: 1,
  dim: [ 3, 2 ],
  data:
   [ 2.7075472, 1.3094337, 3.0416667, 2.0520833, 3.439583, 0.24374998 ] }
```

The s1 data set consists of synthetic data. 5000 points, 2 dimensions, 15 clusters. The program prints out the number of points in the top 3 clusters, and then the 15 points for the detected cluster centers.

```
s1 cluster centers
Cluster #1 has 100 points
Cluster #2 has 325 points
Cluster #3 has 375 points
{ v: 1,
  dim: [ 15, 2 ],
  data:
   [ 186599.4,
     551086.3,
     802634.06,
     324907.56,
     678035.3,
     857998.06,
     398870.1,
     404924,
     338826.94,
     562550.25,
     244654.94,
     847641.6,
     320602.44,
     161521.75,
     417799.6,
     787002.2,
     167856.17,
     347812.66,
     507818.22,
     175610.5,
     852058.44,
     157685.61,
     606380.8,
     574534.94,
     123308.49,
     560501.4,
     617926.6,
     399415.97,
     844029.7,
     631610.5 ] }
```

The dim128 data set consists of synthetic data with high dimensionality. 1024 points, 128 dimensions, 16 clusters. The program prints out the number of points in the top 3 clusters, and then the 16 128-d points for the detected cluster centers.

```
dim128 cluster centers
Cluster #1 has 64 points
Cluster #2 has 64 points
Cluster #3 has 64 points
{ v: 1,
  dim: [ 16, 128 ],
  data:
   [ 213.76562,
     57.546875,
     173,
     103.953125,
     158.37497,
     147.45312,
     66.28126,
     170.35938,
     123.765625,
     45.21875,
     42.078125,
     117.953125,
     ... ] }
```

