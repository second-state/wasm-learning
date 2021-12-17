[English](README-en.md) | [Live demo!](https://sls-website-ap-hongkong-5x0ayxu-1302315972.cos-website.ap-hongkong.myqcloud.com/)

##

## Prerequist

请使用 Ubuntu 20.04 or 20.10 并安装 <a href="https://www.serverless.com/framework/docs/providers/tencent/guide/installation/">Serverless Framework</a>, <a href="https://www.rust-lang.org/tools/install">Rust</a>, and <a href="https://www.secondstate.io/articles/ssvmup/">ssvmup</a>.

或者用 Docker

```
$ docker pull secondstate/tencent-tensorflow-scf
$ docker run --rm -it -v $(pwd):/app secondstate/tencent-tensorflow-scf
(docker) $ cd /app
```

## 创建

运行下面的命令行来创建云函数。

```
$ ssvmup build --enable-aot
```

## 部署

运行下面的命令行将云函数部署到腾讯云上。

```
$ cp pkg/scf.so scf/

$ sls deploy
... ...
  website:       https://sls-website-ap-hongkong-kfdilz-1302315972.cos-website.ap-hongkong.myqcloud.com
```

在浏览器内加载部署好的 URL。 Have fun!
