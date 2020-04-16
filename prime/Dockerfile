FROM rust:latest AS build

WORKDIR /usr/src/prime

COPY . .

RUN cargo build --release

RUN cargo install --path .

FROM alpine:latest

COPY --from=build /usr/local/cargo/bin/prime /bin

ENTRYPOINT ["/bin/prime"]