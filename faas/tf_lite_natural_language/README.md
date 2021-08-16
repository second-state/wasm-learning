# TensorFlow Natural Language

This tutorial demonstrates how to create a a Function as a Service (FaaS), using TensorFlow Natural Language, which is able to classify natural language in terms of connotation/sentiment. 

## Build the WASM bytecode

```
$ rustwasmc build
```

## Create FaaS function

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: mobilenet' \
--data-binary '@pkg/tf_lite_natural_language_bg.wasm'
```

Returns

```
{"wasm_id":392,"wasm_sha256":"0x9f18414daa717c4ea5235623f087c737bd4994c81dd1c0e80ff3d9b5e1f5c029","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"76e6f3dc-15ff-4f4d-a9a0-dd793b1d1c20"}
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/392' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: 76e6f3dc-15ff-4f4d-a9a0-dd793b1d1c20' \
--data-binary '@pkg/tf_lite_natural_language_bg.wasm'
```

## Test

Make a function call via the web.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/392/classify' \
--header 'Content-Type: text/plain' \
--data-raw 'I really like this function, it is brilliant and fun.'
```

## Results

The function call above will result in the following outcome

```
The words you provided have a 0.38152426 negative connotation, and a 0.61847574 positive connotation
```
