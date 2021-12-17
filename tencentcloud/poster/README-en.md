[中文](README.md) | [Live demo!](https://sls-website-ap-hongkong-5x0ayxu-1302315972.cos-website.ap-hongkong.myqcloud.com/)

## Prerequisite

Please use Ubuntu 20.04 or 20.10, and install <a href="https://www.serverless.com/framework/docs/providers/tencent/guide/installation/">Serverless Framework</a>, <a href="https://www.rust-lang.org/tools/install">Rust</a>, and <a href="https://www.secondstate.io/articles/ssvmup/">ssvmup</a>.

Or, use our dev Docker image.

```
$ docker pull secondstate/tencent-tensorflow-scf
$ docker run --rm -it -v $(pwd):/app secondstate/tencent-tensorflow-scf
(docker) $ cd /app
```

## Build

Run the following command to build your cloud function.

```
$ ssvmup build --enable-aot
```

## Deploy

Run the following commands to deploy the cloud function to the Tencent Cloud.

```
$ cp pkg/scf.so scf/

$ sls deploy
... ...
  website:       https://sls-website-ap-hongkong-kfdilz-1302315972.cos-website.ap-hongkong.myqcloud.com
```

Load the deployed URL in any web browser and have fun!

