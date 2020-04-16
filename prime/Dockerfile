FROM rust:latest AS build

WORKDIR /usr/src/prime

COPY . .

RUN cargo build --release

RUN cargo install --path .

FROM scratch

COPY --from=build /usr/local/cargo/bin/prime /

ENTRYPOINT ["/prime"]