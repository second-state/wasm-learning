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
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/watermark.wasm < html/name.json
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/watermark.so < html/name.json
```

## Deploy

Deploy the layer on TencentCloud SCF by uploading the `layer.zip` file via COS.

```
$ cd layer
$ zip --symlinks -r layer.zip *
```

Deploy the function by uploading the `watermark.zip` file to the function service.

```
$ cd cloud
$ cp ../pkg/watermark.so .
$ zip -r watermark.zip *
```

Create and connect an API gateway to the function and turn on CORS.

## Live test

```
$ echo 'Michael Yuan' | curl -d @- -X POST https://service-cjj1420w-1302315972.hk.apigw.tencentcs.com/release/rust_china_conf_watermark
```

## Jamstack test

Upload the `html` files to a static file host. Example: https://tensorflow-demo-0gbniz1314770b82-1302315972.tcloudbaseapp.com/watermark/


