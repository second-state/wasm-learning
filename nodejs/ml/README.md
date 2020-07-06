# The kMeans + SVG example

In this example, we demonstrate how to do kMeans fitting of data, and then plot the results in an SVG graph.

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

Point your browser to http://localhost:8080/, upload a CSV file for 2D data points, and hit the "Draw" button. The web app provides sample CSV files you can download and then upload to process. The birch3 data set is large and will take longer to complete.

## Test

```
$ node node/test.js
<?xml version="1.0" standalone="no"?>
<svg
  ... ...
</svg>
```
