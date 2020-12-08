
Build

```
$ ssvmup build
```

Test

```
$ cloud/ssvm-tensorflow-lite pkg/mobilenet_bg.wasm < html/bird.json
$ cloud/ssvm-tensorflow-lite pkg/mobilenet_bg.wasm < html/PurpleGallinule.json
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
$ base64 -w 0 html/bird.jpg | curl -d @- -X POST https://service-aj0plx8u-1302315972.hk.apigw.tencentcs.com/release/my_hk
$ base64 -w 0 html/PurpleGallinule.jpg | curl -d @- -X POST https://service-aj0plx8u-1302315972.hk.apigw.tencentcs.com/release/my_hk
```
