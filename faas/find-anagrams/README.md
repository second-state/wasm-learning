# The anagram example

In this example, we demonstrate finding anagrams from random letters.

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
--header 'SSVM-Description: Find anagrams' \
--data-binary '@pkg/anagram_solver_lib_bg.wasm'
```

Returns

```
{"wasm_id":237,"wasm_sha256":"0xeaeb73c169f377461a76597a04540a387f8e99d4e08332e42c281f051f9f5393","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"xxx-ac25-48fb-8eed-xxx"} 
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/253' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: xxx-ac25-48fb-8eed-xxx' \
--data-binary '@pkg/anagram_solver_lib_bg.wasm'
```

## Call the function using Javascript byte array

Please click on [this HTML link](https://second-state.github.io/wasm-learning/faas/find-anagrams/html/index.html) which will take you to the live demonstration.

## Call the function 

Searching for valid anagrams, from the letters eoprw (i.e. power)

```
curl --location --request POST 'https://dev.rpc.ssvm.secondstate.io:8081/api/run/253/find_anagrams' \
--header 'Content-Type: text/plain' \
--data-raw 'eoprw'
```


## Articles
This work is explained in more detail in an online article called [](). 

## Further work
This is just a prototype example which can be refined (words list can be updated) to become an API endpoint for a game or anagram app etc.

