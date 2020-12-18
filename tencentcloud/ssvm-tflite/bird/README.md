
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
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/mobilenet.wasm < html/bird.json
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/mobilenet.so < html/bird.json

$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/mobilenet.wasm < html/PurpleGallinule.json
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/mobilenet.so < html/PurpleGallinule.json
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

Live test

```
$ base64 -w 0 html/bird.jpg | curl -d @- -X POST https://service-r7cv20mk-1302315972.hk.apigw.tencentcs.com/release/tflite_bird
$ base64 -w 0 html/PurpleGallinule.jpg | curl -d @- -X POST https://service-r7cv20mk-1302315972.hk.apigw.tencentcs.com/release/tflite_bird
```
