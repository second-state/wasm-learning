# The search bytes example

In this example, we demonstrate how to see if a sequence of bytes is present in a specific byte array and/or if two byte arrays are an exact match.

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
--header 'SSVM-Description: Search bytes' \
--data-binary '@pkg/search_bytes_lib_bg.wasm'
```

Returns

```
{"wasm_id":225,"wasm_sha256":"0xe6d0a5d7e21841dcd1c6694d0ba7e9741ede296806edf41c093d6822a93c48e8","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"xxxxxxx-xxxxx-xx-xx-x-3fad2948a604"}
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/225' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: xxxxxxx-xxxxx-xx-xx-x-3fad2948a604' \
--data-binary '@pkg/search_bytes_lib_bg.wasm'
```

## Call the function using multipart formdata

Comparing two images (to see if they are an exact match at the byte level)

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/multipart/run/225/search_bytes' \
--form 'input_2=@/home/pictures/haystack.png' \
--form 'input_1=@/home/pictures/needle.png'
```
If the bytes from the `needle.png` image are present in the `haystack.png` image (i.e. they match) then the function will return
```
Present
``` 
If the bytes from the `needle.png` image are **NOT** present in the `haystack.png` image (i.e. they match) then the function will return
```
Absent
```

## Call the function using Javascript byte array
225
