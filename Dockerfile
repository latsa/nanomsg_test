FROM rust:latest
COPY ./ /app
RUN apt-get update
RUN apt-get install -y build-essential make git curl cmake sudo apt-utils
WORKDIR /app/deps/nanomsg.rs
RUN make deps
WORKDIR /app
RUN cargo build --release \
    && cp target/release/nanomsg_test . \
    && rm -rf target/ ~/.cargo/
ENTRYPOINT ["./nanomsg_test"]
