FROM rust:1.53

WORKDIR /usr/src/server
COPY . .

RUN cargo install --path .
RUN cargo build --release

ENTRYPOINT [ "./target/release/cubby_server" ]
