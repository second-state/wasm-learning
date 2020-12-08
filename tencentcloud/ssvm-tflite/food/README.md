
Build

```
$ ssvmup build
```

Test

```
$ cloud/ssvm-tensorflow-lite pkg/mobilenet_bg.wasm < html/food.json
```

Deploy

```
$ cd cloud
$ cp ../pkg/mobilenet_bg.wasm .
$ zip mobilenet.zip *
```

Now you can upload the `mobilenet.zip` file to TencentCloud.

Live test

```
$ base64 -w 0 html/food.jpg | curl -d @- -X POST https://service-lty62pd6-1302315972.hk.apigw.tencentcs.com/release/tflite_food
```
