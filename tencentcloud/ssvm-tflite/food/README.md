
## Prerequisite

[Install the ssvmup tool](https://www.secondstate.io/articles/ssvmup/)

## Build

```
$ ssvmup build --enable-aot
```

## Test

Create the layer for tensorflow and SSVM binaries.

```
$ cd ../layer
$ source download_dependencies.sh
```

Run the wasm application.

```
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/mobilenet.wasm < html/food.json
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/mobilenet.so < html/food.json
```

## Deploy

Deploy the layer on TencentCloud SCF by uploading the `layer.zip` file via COS.

```
$ cd layer
$ zip --symlinks -r layer.zip *
```

Deploy the function by uploading the `mobilenet.zip` file to the function service.

```
$ cd cloud
$ cp ../pkg/mobilenet.so .
$ zip -r mobilenet.zip *
```

Create and connect an API gateway to the function and turn on CORS.

## Live test

```
$ base64 -w 0 html/food.jpg | curl -d @- -X POST https://service-lty62pd6-1302315972.hk.apigw.tencentcs.com/release/tflite_food
```

## Jamstack test

Upload the `html` files to a static file host. Example: https://tensorflow-demo-0gbniz1314770b82-1302315972.tcloudbaseapp.com/food/


