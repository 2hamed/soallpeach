FROM rust:latest AS build

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/prime

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

RUN cargo install --path . --target x86_64-unknown-linux-musl

FROM alpine:latest

COPY --from=build /usr/local/cargo/bin/prime /bin

ENTRYPOINT [ "/bin/prime" ]