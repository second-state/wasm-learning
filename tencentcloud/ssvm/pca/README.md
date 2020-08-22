# The Principal Component Analysis (PCA) + SVG example

In this example, we demonstrate how to do Principal Component Analysis (PCA) of a 2-D data array, and then plot the results in an SVG graph.

> To draw SVG graphs in Rust, we used the techniques described by [Cetra](https://cetra3.github.io/blog/) in [this article](https://cetra3.github.io/blog/drawing-svg-graphs-rust/).

## Prerequisites

If you have not done so already, follow these simple instructions to [install ssvmup](https://www.secondstate.io/articles/ssvmup/) on your development computer.

## Build the WASM bytecode

```
$ ssvmup build
```

## Package for deployment

```
$ cp pkg/pca_bg.wasm cloud/
$ cd cloud
$ zip pca.zip *
```

## Deploy on TencentCloud

[Follow the instructions here](https://github.com/second-state/ssvm-tencent-starter/blob/master/README.md#deploy-on-tencentcloud). Make sure that the "execution handler" is set to `pca_bg.wasm` in the web console.

## Create a web service

Go to Trigger Management, and Create Trigger. Select API Gateway Trigger from the list, and Submit.

![Create a web API trigger](docs/create.png)

Copy down the access path URL once the web API trigger is created.

![URL to access the web API](docs/access.png)

## Test

Next, go to the `test` folder and use `curl` to post a CSV data file to the access path URL. It should return a SVG graph plotting the input 2D points as well as the two principal components.

```
$ cd test
$ curl -d @iris.csv -X POST https://service-m9pxktbc-1302315972.hk.apigw.tencentcs.com/release/PCASVG
```

