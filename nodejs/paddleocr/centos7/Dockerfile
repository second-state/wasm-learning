FROM centos:7
ARG DEBIAN_FRONTEND=noninteractive
RUN yum install -y git
RUN yum install -y libtool
RUN yum install clang
RUN yum install -y gcc-c++.x86_64
RUN yum install -y zlib
RUN yum install -y zlib-devel
RUN yum install -y libjpeg
RUN yum install -y libjpeg-devel
RUN yum install -y libwebp
RUN yum install -y libwebp-devel
RUN yum install -y libtiff
RUN yum install -y libtiff-devel
RUN yum install -y libpng
RUN yum install -y libpng-devel
RUN yum install python-devel
RUN yum install python3-devel
RUN yum install libevent-devel
RUN yum install python3-pip
RUN yum install patch
RUN yum -y install wget make gcc-c++
RUN yum install libXext libSM libXrender
RUN cd /usr/local/lib
RUN cp /usr/lib64/libjpeg.so.62 .
RUN cp /usr/lib64/libwebp.so.4 .
RUN cp /usr/lib64/libtiff.so.5 .
RUN cp /usr/lib64/libpng15.so.15 .
RUN wget -O - 'https://ftpmirror.gnu.org/gcc/gcc-7.3.0/gcc-7.3.0.tar.xz' | tar -xJ
RUN ( cd gcc-7.3.0 && ./contrib/download_prerequisites && mkdir build && cd build && ../configure --enable-checking=release --enable-languages=c,c++ --disable-multilib && make -j 8 &&  make install ) && rm -fR gcc-7.3.0
RUN unlink /usr/lib64/libstdc++.so.6
RUN cp /usr/local/lib64/libstdc++.so.6 /usr/lib64
RUN pip3 install paddlepaddle && sudo pip3 install paddleocr
