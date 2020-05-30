# The hello example

In this example, we demonstrate how to create and run a Rust function in the Second State Rust FaaS.

## Set up

[See it here](https://cloud.secondstate.io/function-as-a-service/getting-started).

## Write Rust code

Below is the entire content of the [src/lib.rs](src/lib.rs) file.

```
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(context: &str, s: &str) -> String {
  let r = String::from("hello ");
  let ret = "
    {
      'callback': {
        'method': 'POST',
        'hostname': 'api.sendgrid.com',
        'port': 443,
        'path': '/v3/mail/send',
        'headers': {
          'Content-Type': 'application/json',
          'authorization': 'Bearer AUTH_TOKEN'
        },
        'maxRedirects': 20
      },
      'personalizations': {
        [{
          'to':[{'email':'TO_EMAIL','name':''}],
          'subject':'SUBJECT'
        }],
        'from':{'email':'FROM_EMAIL','name':''}
      }
    }
  ";
  
  let ret = ret.replace("AUTH_TOKEN", "auth_token_123");
  let ret = ret.replace("TO_EMAIL", "alice@secondstate.io");
  let ret = ret.replace("SUBJECT", &(r + &s));
  let ret = ret.replace("FROM_EMAIL", "dev@developer.com");
  return ret;
}
```

## Build the WASM bytecode

```
$ ssvmup build --nowasi
```

## FaaS

Upload the wasm file to the FaaS.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: say hello' \
--data-binary 'pkg/hello_lib_bg.wasm'
{"wasm_id":123}
```

Make a function call via the web.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/123/say' \
--header 'Content-Type: text/plain' \
--data-raw 'Second State FaaS'
```

The `TO_EMAIL` address in the function will now receive an email message with the "hello Second State FaaS" subject line.


