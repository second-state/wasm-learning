## Prerequisite 前期准备

[Install the ssvmup tool](https://www.secondstate.io/articles/ssvmup/)
and the Serverless Framework.
[安装 ssvmup 工具](https://www.secondstate.io/articles/ssvmup/)
和 Serverless 框架。

## Build 创建

```
$ ssvmup build --enable-aot
```

## Test 测试

Create the layer for tensorflow and SSVM binaries.
为 tensorflow 和 SSVM 二进制文件创建 layer。

```
$ cd ../layer
$ source download_dependencies.sh
```

Run the wasm application. 
运行wasm应用。

```
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/scf.wasm < test/name.json
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/scf.so < test/name.json
```

## Deploy 部署

```
$ cp pkg/scf.so scf/
$ sls deploy
```

Test the Jamstack web app via the website URL created from the above step.
通过以上步骤创建的网站 URL 测试 Jamstack web 应用。

