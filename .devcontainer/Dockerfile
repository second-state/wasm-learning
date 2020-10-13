FROM ubuntu:20.04
ARG DEBIAN_FRONTEND=noninteractive
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN apt-get update \
    && apt-get -y upgrade \
    && apt install -y build-essential libboost-all-dev \
    && apt install -y curl wget git vim pkg-config libssl-dev
RUN curl -sL https://deb.nodesource.com/setup_14.x |  bash \
    && apt-get install -y nodejs \
    && npm install -y -g ssvmup --unsafe-perm \
    && npm install -y ssvm \
    && npm install -y express express-fileupload
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN wget https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-gpu-linux-x86_64-1.15.0.tar.gz \
    && tar -C /usr/ -xzf libtensorflow-gpu-linux-x86_64-1.15.0.tar.gz
RUN cargo install http_proxy \
    && cargo install mtcnn \
    && cargo install mobilenet_v2

# Switch back to dialog for any ad-hoc use of apt-get
ENV DEBIAN_FRONTEND=dialog
