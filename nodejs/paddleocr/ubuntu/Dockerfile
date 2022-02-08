FROM ubuntu:20.04
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update\
    && apt-get upgrade && apt-get install python3.7 \
    && apt-get install python3-pip
RUN ln -s python3.7 /usr/bin/python3
RUN pip3 install  numpy==1.20.0 && pip3 install paddlepaddle && pip3 install paddleocr
ENV DEBIAN_FRONTEND=dialog
