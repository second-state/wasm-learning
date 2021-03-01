
# The paddlepaddleocr example

In this example, we will demonstrate how to detect, classify, and recognize characters from an image i.e. Optical Character Recognition (OCR).

## Prerequisites
Before you do the following instructions, 
if your system is centos7,you must install:
```
sudo yum install -y git
sudo yum install -y libtool
sudo yum install clang
sudo yum install -y gcc-c++.x86_64
sudo yum install -y zlib
sudo yum install -y zlib-devel
sudo yum install -y libjpeg
sudo yum install -y libjpeg-devel
sudo yum install -y libwebp
sudo yum install -y libwebp-devel
sudo yum install -y libtiff
sudo yum install -y libtiff-devel
sudo yum install -y libpng
sudo yum install -y libpng-devel
sudo yum install python-devel
sudo yum install python3-devel
sudo yum install libevent-devel
sudo yum install patch
sudo yum -y install wget make gcc-c++
sudo yum install libXext libSM libXrender
cd /usr/local/lib
sudo cp /usr/lib64/libjpeg.so.62 .
sudo cp /usr/lib64/libwebp.so.4 .
sudo cp /usr/lib64/libtiff.so.5 .
sudo cp /usr/lib64/libpng15.so.15 .
wget -O - 'https://ftpmirror.gnu.org/gcc/gcc-7.3.0/gcc-7.3.0.tar.xz' | tar -xJ
( cd gcc-7.3.0 && ./contrib/download_prerequisites && mkdir build && cd build && ../configure --enable-checking=release --enable-languages=c,c++ --disable-multilib && make -j 8 && sudo make install ) && rm -fR gcc-7.3.0
sudo unlink /usr/lib64/libstdc++.so.6
sudo cp /usr/local/lib64/libstdc++.so.6 /usr/lib64
sudo pip3 install paddlepaddle && sudo pip3 install paddleocr
```
and if you OS is ubuntu20.04,you must #first# install
```
pip3 install python3.7
pip3 install numpy==1.14.6
pip3 install paddleocr
```
Notes:If you don't use the version mentioned above, error comes.
If you have not done so already, follow these simple instructions to install [Rust](https://www.rust-lang.org/tools/install) and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

## Build the WASM bytecode
Notice that we are using the `--enable-ext` flag which will use `ssvm-extensions` instead of `ssvm`.

```
$ ssvmup build --enable-aot --enable-ext
```



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
text() function returns
["(\'We would like to thank all the designers and"]
["contributors who have been involved in the"]
["production of this book; their contributions"]
["have been indispensable to its creation. We"]
["would also like to express our gratitude to all"]
["the producers for their invaluable opinions"]
["and assistance throughout this project.And to"]
["the many others whose names are not credited"]
["but have made specific input in this book， we"]
["thank you for your continuous support."]


obtained by function call # bounding_box()
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


```
