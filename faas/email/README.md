# The email example

In this example, we will demonstrate how to redirect the return value from a FaaS function to another online service.

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Build the WASM bytecode

```
$ ssvmup build
```

## Create FaaS function

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: send email' \
--data-binary '@pkg/send_email_lib_bg.wasm'
```

Returns

```
{"wasm_id":123,"wasm_sha256":"0xec9e4c7d01920f...644bed9bf7922","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"b425089...8bfa58e6"}
```

## Set from address

Set the context state to the `from` address for functions in this wasm file.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/state/123' --header 'Content-Type: text/plain' --data 'michael@secondstate.io'
```

## Redirect results to another service

This is done by associating a callback object with the wasm file. The callback is a HTTP request object in JSON format. The function call's return value is submiited to the callback upon completion.

```
curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/callback/123' \
--header 'Content-Type: application/json' \
--data-raw '{"hostname": "api.sendgrid.com","path": "/v3/mail/send","method": "POST","port": 443,"headers":{"Content-Type": "application/json","authorization": "Bearer SG.xxxx"}}'
```

## Test

Make a function call via the web.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/send_email' \
--header 'Content-Type: text/plain' \
--data '{"to":"michael@michaelyuan.com","subject":"Email from Joey","mime":"text/plain","body":"This email is sent from the Joey FaaS service."}'
```

The `to/email` address in the Rust function will now receive an email message with the "hello Second State FaaS" subject line.

