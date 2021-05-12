FROM ubuntu:20.04
ARG DEBIAN_FRONTEND=noninteractive
RUN sudo rm /usr/bin/python3
RUN apt-get update\
    && apt-get upgrade && apt-get install python3.7 && apt-get install  numpy==1.14.6\
    && apt-get install paddlepaddle && apt-get install python3-pip
RUN sudo ln -s python3.7 /usr/bin/python3
RUN pip3 install paddlepaddle && pip3 install paddleocr
ENV DEBIAN_FRONTEND=dialog
