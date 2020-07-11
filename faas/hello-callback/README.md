# The callback example

In this example, we will demonstrate how to redirect the return value from a FaaS function to another online service.

## Set up

[See it here](https://cloud.secondstate.io/function-as-a-service/getting-started).

## Write Rust code

Below is the entire content of the [src/lib.rs](src/lib.rs) file.

```
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let r = String::from("hello ");
  let ret = "
    {
      'personalizations': {
        [{
          'to':[{'email':'TO_EMAIL','name':''}],
          'subject':'SUBJECT'
        }],
        'from':{'email':'FROM_EMAIL','name':''}
      }
    }
  ";
  
  let ret = ret.replace("TO_EMAIL", "juntao_yuan@yahoo.com");
  let ret = ret.replace("SUBJECT", &(r + &s));
  let ret = ret.replace("FROM_EMAIL", "michael@secondstate.io");
  return ret;
}
```

## Build the WASM bytecode

```
$ ssvmup build
```

## Create FaaS function

Upload the wasm file to the FaaS.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: say hello' \
--data-binary @'pkg/hello_lib_bg.wasm'
{"wasm_id":123}
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
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/say' \
--header 'Content-Type: text/plain' \
--data-raw 'Second State FaaS'
```

The `TO_EMAIL` address in the function will now receive an email message with the "hello Second State FaaS" subject line.




