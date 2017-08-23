FROM alpine:3.6 AS builder
RUN apk add --no-cache cargo rust
COPY . /root/
RUN cargo build --release --manifest-path=/root/Cargo.toml

FROM alpine:3.6
RUN apk add --no-cache curl llvm-libunwind
WORKDIR /root
COPY ./static /root/static
COPY --from=builder /root/target/release/main /root

CMD /root/main
ENV ADDR 0.0.0.0:80
HEALTHCHECK --interval=1m --timeout=3s CMD curl -f http://localhost/ || exit 1
