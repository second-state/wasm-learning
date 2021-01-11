# The OCR example

In this example, we will demonstrate how to detect characters from an image i.e. Optical Character Recognition (OCR).

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Build the WASM bytecode
Notice that we are using the `--enable-ext` flag which will use `ssvm-extensions` instead of `ssvm`.

```
$ ssvmup build
```

## Create FaaS function

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: Translate' \
--data-binary '@pkg/translate_lib_bg.wasm'
```

Returns

```
{"wasm_id":283,"wasm_sha256":"0x39cfdbe0d0aa31d87e81d72506fa88af5ab6f3ba82b3d09f5330aac8ba061673","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"xxxxxx-de44-4fc8-abf7-03f61f648b71"}
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/283' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: xxxxxx-de44-4fc8-abf7-03f61f648b71' \
--data-binary '@pkg/translate_lib_bg.wasm'
```

## Test

Make a function call via the web.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/multipart/run/283/translate' \
--form 'input_1="Saluton, Mondo"' \
--form 'input_2="en"'
```

Returns
```
Hello World
```

## Output language options

If you are building a web app, please specify the language which you want the input translated to via your form like this
```
<div class="form-group">
  <label for="input_3">Would you like to translate the input to another language?</label>
  <fieldset>
     <legend>Output translation options</legend>
     <p>
        <label>Output translation options</label>
        <select id = "input_3">
           <option value = "af">Afrikaans</option>
           <option value = "ar">Arabic</option>
           <option value = "be">Belarusian</option>
           <option value = "bn">Bengali</option>
           <option value = "bs">Bosnian</option>
           <option value = "bg">Bulgarian</option>
           <option value = "cs">Czech</option>
           <option value = "zh-CN">Chinese simplified</option>
           <option value = "zh-TW">Chinese traditional</option>
           <option value = "en" selected>English</option>
           <option value = "fr">French</option>
           <option value = "ja">Japanese</option>
           <option value = "ko">Korean</option>
           <option value = "ru">Russian</option>
           <option value = "uk">Ukrainian</option>
        </select>
     </p>
  </fieldset>
</div>
```

## Live demo

Please click on [this HTML link](https://second-state.github.io/wasm-learning/faas/language-translation/html/index.html) which will take you to the live demonstration.
