# The MTCNN example

Run MTCNN tensorflow models as functions.

[Live Demo](https://second-state.github.io/wasm-learning/faas/mtcnn/html/index.html) | [Code Tutorial](https://www.secondstate.io/articles/faas-face-detection/)

## Prerequisites

If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [rustwasmc](https://www.secondstate.io/articles/ssvmup/).

## Build the WASM bytecode

```
$ rustwasmc build --enable-ext
```

## Create FaaS function

Upload the wasm file in the `pkg` folder to the FaaS. Double check the `.wasm` file name before you upload.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/executables' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM-Description: mtcnn' \
--data-binary '@pkg/mtcnn_service_lib_bg.wasm'
```

Returns

```
{"wasm_id":482,"wasm_sha256":"0x469c28daae7aba392076b4bc5ee3b43ec6d667083d8ae63207bf74b1da03fc26","SSVM_Usage_Key":"00000000-0000-0000-0000-000000000000","SSVM_Admin_Key":"7dxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx0c41"}
```

Note: You can update this binary with the `SSVM_Admin_Key`.

```
$ curl --location --request PUT 'https://rpc.ssvm.secondstate.io:8081/api/update_wasm_binary/195' \
--header 'Content-Type: application/octet-stream' \
--header 'SSVM_Admin_Key: 7dxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx0c41' \
--data-binary '@pkg/mtcnn_service_lib_bg.wasm'
```

## Test

Make a function call via the web.

```
$ curl --location --request POST 'https://rpc.ssvm.secondstate.io:8081/api/run/482/infer/bytes' \
--header 'Content-Type: application/octet-stream' \
--data-binary '@test/solvay.jpg' \
--output tmp.jpg
```

## Local test

You must have Node.js and NPM installed. Install dependencies.

```
$ sudo apt install -y libjpeg-dev libpng-dev
$ wget https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-cpu-linux-x86_64-2.3.0.tar.gz
$ sudo tar -C /usr/local -xzf libtensorflow-cpu-linux-x86_64-2.3.0.tar.gz
$ sudo ldconfig
```

Install wasmedge-extensions from source

You will need to alter the Git configuration on the machine where this installation procedure is being performed. Reason being, this machine will not have the SSH keys to communicate with git@github. If you add the following config, you will be able to successfully run the `npm install --build-from-source ...` below.

```bash
git config --global url."https://github.com/".insteadOf git@github.com:
git config --global url."https://".insteadOf git://
```

You can check that this config worked by typing 

```bash
git config -l
```

Temporarily downgrade npm

The following installation will require that npm is downgraded to `6.14.9`. There is [an npm cli issue](https://github.com/npm/cli/issues/1865) which prevents us from using the latest npm for this particular build from source task.

Because of the complexity of dependency management, please install aptitude because it provides a way to automatically resolve depencency conflicts.

```bash
sudo apt install aptitude
sudo aptitude install npm
```

The following command is used to alter the npm version.

```bash
sudo npm install -g npm@6.14.9
```

Once you have temporarily downgraded npm, please go ahead and install the latest wasmedge-extensions like this

```bash
git clone --recurse-submodules https://github.com/second-state/wasmedge-extensions.git
npm install --build-from-source wasmedge-extensions
```

Run the local test on Node.js.

```bash
$ cd test
$ node test.js
Drawing box: 30 results ...
Face Detection: 888.961ms
```


