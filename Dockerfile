FROM ubuntu:xenial

RUN apt-get update
RUN apt-get install -yq sudo curl wget git file g++ cmake pkg-config \
                        bison flex unzip lib32stdc++6 lib32z1

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH /root/.cargo/bin:$PATH

COPY . /root/

RUN cargo build --release --manifest-path=/root/Cargo.toml
RUN mv /root/target/release/main /
RUN rm -rf /root/*

WORKDIR /
CMD ./main
