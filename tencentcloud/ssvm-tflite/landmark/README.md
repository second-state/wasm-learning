[English](README-en.md)

## 前期准备

[安装 ssvmup 工具](https://www.secondstate.io/articles/ssvmup/)和 Serverless 框架。

## 创建

```
$ ssvmup build --enable-aot
```

## 测试

为 tensorflow 和 SSVM 二进制文件创建 layer。

```
$ cd ../layer
$ source download_dependencies.sh
```

运行 wasm 应用。

```
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/scf.wasm < test/fuji.json
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/scf.so < test/fuji.json

$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/scf.wasm < test/taipei101.json
$ LD_LIBRARY_PATH=../layer ../layer/ssvm-tensorflow pkg/scf.so < test/taipei101.json
```

## 部署

```
$ cp pkg/scf.so scf/

$ cd cos
$ wget https://tensorflow-dep-1302315972.cos.ap-hongkong.myqcloud.com/layer.zip
$ cd ..

$ sls deploy
```

通过以上步骤创建的网站 URL 测试 Jamstack web 应用。

