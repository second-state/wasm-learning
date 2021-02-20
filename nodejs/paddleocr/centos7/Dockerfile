FROM centos:7
ARG DEBIAN_FRONTEND=noninteractive
RUN sudo yum install -y git
RUN sudo yum install -y libtool
RUN sudo yum install clang
RUN sudo yum install -y gcc-c++.x86_64
RUN sudo yum install -y zlib
RUN sudo yum install -y zlib-devel
RUN sudo yum install -y libjpeg
RUN sudo yum install -y libjpeg-devel
RUN sudo yum install -y libwebp
RUN sudo yum install -y libwebp-devel
RUN sudo yum install -y libtiff
RUN sudo yum install -y libtiff-devel
RUN sudo yum install -y libpng
RUN sudo yum install -y libpng-devel
RUN sudo yum install python-devel
RUN sudo yum install python3-devel
RUN sudo yum install libevent-devel
RUN sudo yum install patch
RUN sudo yum -y install wget make gcc-c++
RUN sudo yum install libXext libSM libXrender
RUN cd /usr/local/lib
RUN sudo cp /usr/lib64/libjpeg.so.62 .
RUN sudo cp /usr/lib64/libwebp.so.4 .
RUN sudo cp /usr/lib64/libtiff.so.5 .
RUN sudo cp /usr/lib64/libpng15.so.15 .
RUN wget -O - 'https://ftpmirror.gnu.org/gcc/gcc-7.3.0/gcc-7.3.0.tar.xz' | tar -xJ
RUN ( cd gcc-7.3.0 && ./contrib/download_prerequisites && mkdir build && cd build && ../configure --enable-checking=release --enable-languages=c,c++ --disable-multilib && make -j 8 && sudo make install ) && rm -fR gcc-7.3.0
RUN sudo unlink /usr/lib64/libstdc++.so.6
RUN sudo cp /usr/local/lib64/libstdc++.so.6 /usr/lib64
RUN sudo pip3 install paddlepaddle && sudo pip3 install paddleocr
