# The paddlepaddleocr example

In this example, we will demonstrate how to detect, classify, and recognize characters from an image i.e. Optical Character Recognition (OCR).

## Prerequisites

Before you do the following instructions, you must install:

```
$ pip3 install python3.7
$ pip3 install numpy==1.14
$ pip3 install paddleocr
```

Notes:If you don't use the version mentioned above, error comes.
If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Build the WASM bytecode

Notice that we are using the `--enable-ext` flag which will use `ssvm-extensions` instead of `ssvm`.

```
$ ssvmup build --enable-aot --enable-ext
```

## Create FaaS function

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: ocr' \
--data-binary '@pkg/ocr_lib_bg.wasm'
```

Returns

```
{"wasm_id":310,"wasm_sha256":"0x39cfdbe0d0aa31d87e81d72506fa88af5ab6f3ba82b3d09f5330aac8ba061673","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"xxxxxx-de44-4fc8-abf7-03f61f648b71"}
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/310' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: xxxxxx-de44-4fc8-abf7-03f61f648b71' \
--data-binary '@pkg/ocr_lib_bg.wasm'
```

## Test1

Make a function call via the web.

```
curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/multipart/run/310/ocr' \
--form 'input_1=@"test/img_12.jpg"' \
```

## Test2

 via the Nodejs.
 You must have Node.js and NPM installed. Install SSVM extensions and dependencies.

```
$ sudo apt install -y libjpeg-dev libpng-dev
$ wget https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-cpu-linux-x86_64-2.3.0.tar.gz
$ sudo tar -C /usr/local -xzf libtensorflow-cpu-linux-x86_64-2.3.0.tar.gz
$ sudo ldconfig
$ npm i ssvm-extensions
```

Run the local test on Node.js.

```
$ cd test
$ node test.js
obtained by function call # only_detection
root INFO: [[394.0, 748.0], [1212.0, 746.0], [1213.0, 789.0], [394.0, 791.0]]
root INFO: [[394.0, 698.0], [1213.0, 692.0], [1214.0, 737.0], [394.0, 743.0]]
root INFO: [[395.0, 643.0], [1213.0, 642.0], [1213.0, 691.0], [395.0, 692.0]]
root INFO: [[393.0, 594.0], [1214.0, 588.0], [1215.0, 639.0], [393.0, 644.0]]
root INFO: [[396.0, 543.0], [1214.0, 542.0], [1214.0, 590.0], [396.0, 592.0]]
root INFO: [[397.0, 495.0], [1210.0, 495.0], [1210.0, 537.0], [397.0, 537.0]]
root INFO: [[396.0, 442.0], [1212.0, 440.0], [1213.0, 491.0], [396.0, 493.0]]
root INFO: [[398.0, 394.0], [1207.0, 393.0], [1208.0, 439.0], [398.0, 440.0]]
root INFO: [[396.0, 342.0], [1209.0, 344.0], [1208.0, 390.0], [396.0, 388.0]]
root INFO: [[437.0, 170.0], [1170.0, 173.0], [1170.0, 227.0], [437.0, 224.0]]

obtained by function call # detection_classification_and_recognition
root INFO: dt_boxes num : 11, elapse : 2.5862977504730225
root INFO: cls num  : 11, elapse : 0.11584687232971191
root INFO: rec_res num  : 11, elapse : 0.684161901473999
root INFO: [[[437.0, 170.0], [1170.0, 173.0], [1170.0, 227.0], [437.0, 224.0]], (\'ACKNOWLEDGEMENTS\', 0.95431674)]
root INFO: [[[396.0, 342.0], [1209.0, 344.0], [1208.0, 390.0], [396.0, 388.0]], (\'We would like to thank all the designers and\', 0.97767454)] 
root INFO: [[[398.0, 394.0], [1207.0, 393.0], [1208.0, 439.0], [398.0, 440.0]], (\'contributors who have been involved in the\', 0.97794926)]
root INFO: [[[396.0, 442.0], [1212.0, 440.0], [1213.0, 491.0], [396.0, 493.0]], (\'production of this bookj their contributions\', 0.9809355)]
root INFO: [[[397.0, 495.0], [1210.0, 495.0], [1210.0, 537.0], [397.0, 537.0]], (\'have been indispensable to its creation We\', 0.97727674)]
root INFO: [[[396.0, 543.0], [1214.0, 542.0], [1214.0, 590.0], [396.0, 592.0]], (\'would also like to express our gratitude to all\', 0.9904165)]
root INFO: [[[393.0, 594.0], [1214.0, 588.0], [1215.0, 639.0], [393.0, 644.0]], (\'the producers for their invaluable opinions\', 0.9899933)]
root INFO: [[[395.0, 643.0], [1213.0, 642.0], [1213.0, 691.0], [395.0, 692.0]], (\'and assistance throughout this project And to\', 0.9652537)]
root INFO: [[[394.0, 698.0], [1213.0, 692.0], [1214.0, 737.0], [394.0, 743.0]], (\'the many others whose names are not credited\', 0.99038166)]
root INFO: [[[394.0, 748.0], [1212.0, 746.0], [1213.0, 789.0], [394.0, 791.0]], (\'but have made specific input in this book we\', 0.96071094)]
root INFO: [[[394.0, 798.0], [1091.0, 800.0], [1091.0, 846.0], [394.0, 844.0]], (\'thank you for your continuous support\', 0.9937853)]

obtained by function call # detection_and_recognition
root INFO: dt_boxes num : 11, elapse : 2.5341386795043945\n[2021/02/02 02:25:16] root INFO: rec_res num  : 11, elapse : 0.6770017147064209
root INFO: [[[437.0, 170.0], [1170.0, 173.0], [1170.0, 227.0], [437.0, 224.0]], (\'ACKNOWLEDGEMENTS\', 0.95431674)]
root INFO: [[[396.0, 342.0], [1209.0, 344.0], [1208.0, 390.0], [396.0, 388.0]], (\'We would like to thank all the designers and\', 0.97767454)]
root INFO: [[[398.0, 394.0], [1207.0, 393.0], [1208.0, 439.0], [398.0, 440.0]], (\'contributors who have been involved in the\', 0.97794926)]
root INFO: [[[396.0, 442.0], [1212.0, 440.0], [1213.0, 491.0], [396.0, 493.0]], (\'production of this bookj their contributions\', 0.9809355)]
root INFO: [[[397.0, 495.0], [1210.0, 495.0], [1210.0, 537.0], [397.0, 537.0]], (\'have been indispensable to its creation We\', 0.97727674)]
root INFO: [[[396.0, 543.0], [1214.0, 542.0], [1214.0, 590.0], [396.0, 592.0]], (\'would also like to express our gratitude to all\', 0.9904165)]
root INFO: [[[393.0, 594.0], [1214.0, 588.0], [1215.0, 639.0], [393.0, 644.0]], (\'the producers for their invaluable opinions\', 0.9899933)]
root INFO: [[[395.0, 643.0], [1213.0, 642.0], [1213.0, 691.0], [395.0, 692.0]], (\'and assistance throughout this project And to\', 0.9652537)]
root INFO: [[[394.0, 698.0], [1213.0, 692.0], [1214.0, 737.0], [394.0, 743.0]], (\'the many others whose names are not credited\', 0.99038166)]
root INFO: [[[394.0, 748.0], [1212.0, 746.0], [1213.0, 789.0], [394.0, 791.0]], (\'but have made specific input in this book we\', 0.96071094)]
root INFO: [[[394.0, 798.0], [1091.0, 800.0], [1091.0, 846.0], [394.0, 844.0]], (\'thank you for your continuous support\', 0.9937853)]
```

