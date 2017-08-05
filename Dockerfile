FROM alpine:3.6
RUN apk add --no-cache cargo rust

COPY . /root/

RUN cargo build --release --manifest-path=/root/Cargo.toml
RUN mv /root/target/release/main /
RUN mv /root/static /
RUN rm -rf /root/*

ENV ADDR 0.0.0.0:80
HEALTHCHECK --interval=5m --timeout=3s CMD curl -f http://localhost/ || exit 1

WORKDIR /
CMD ./main
