# The base64 example

In this example, we demonstrate how to create and run a Rust function in the Second State Rust FaaS.

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Build the WASM bytecode

```
$ ssvmup build
```

## FaaS

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: base64' \
--data-binary '@pkg/base64_conv_lib_bg.wasm'
```

Returns

```
{"wasm_id":123,"wasm_sha256":"0xfb413547a8aba56d0349603a7989e269f3846245e51804932b3e02bc0be4b665","usage_key":"00000000-0000-0000-0000-000000000000","admin_key":"00xxxxxx-xxxx-xxxx-xxxx-4adc960fd2b8"}
```

Make a function

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/encode' \
--header 'Content-Type: application/octet-stream' \
--data-binary '@test/one_pixel_png_image.png'
```

The above command creates the following text string
```
iVBORw0KGgoAAAANSUhEUgAAAAEAAAABAQMAAAAl21bKAAAAA1BMVEX/TQBcNTh/AAAAAXRSTlPM0jRW/QAAAApJREFUeJxjYgAAAAYAAzY3fKgAAAAASUVORK5CYII=
```

Make a reverse function call via the web, using the text from above as input

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/decode' \
--header 'Content-Type: text/plain' \
--data-raw 'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABAQMAAAAl21bKAAAAA1BMVEX/TQBcNTh/AAAAAXRSTlPM0jRW/QAAAApJREFUeJxjYgAAAAYAAzY3fKgAAAAASUVORK5CYII=' --output tmp.png
```

Confirm the conversion.

```
diff tmp.png test/one_pixel_png_image.png
```

## USE url_safe mode

Set the context state to string `url_safe` for functions in this wasm file.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/state/123' --header 'Content-Type: text/plain' --data-raw 'url_safe'
```

Make a function call via the web.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/encode' \
--header 'Content-Type: application/octet-stream' \
--data-binary '@test/one_pixel_png_image.png'
```

Make a reverse function call via the web.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/decode/bytes' \
--header 'Content-Type: text/plain' \
--data-raw 'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABAQMAAAAl21bKAAAAA1BMVEX/TQBcNTh/AAAAAXRSTlPM0jRW/QAAAApJREFUeJxjYgAAAAYAAzY3fKgAAAAASUVORK5CYII=' --output tmp.png
```

Confirm the conversion.

```
diff tmp.png test/one_pixel_png_image.png
```



