#FROM rust:1.66 as builder

##RUN cargo install --path .
#WORKDIR /src
#COPY ["Cargo.lock", "Cargo.toml", ".env", "config.yml", "./src/"]

##RUN mkdir .cargo
##RUN cargo vendor > .cargo/config

#COPY . /src/

#RUN cargo build
#RUN cargo install --path . --verbose


#FROM debian:bullseye-slim


#COPY --from=builder ./config.yml ./actix-sqlx
#COPY --from=builder ./.env  ./actix-sqlx
#COPY --from=builder ./target/release/actix-sqlx  ./actix-sqlx

#CMD ["./actix-sqlx"]

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

RUN cargo install cargo-chef 

WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
#RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo chef cook --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin app

## We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/app /usr/local/bin
COPY --from=builder /app/config.yml /app
COPY --from=builder /app/.env /app
RUN apt-get update
RUN apt-get install -y wget
RUN apt-get install -y build-essential
RUN apt-get install -y zlib1g-dev
ARG OPENSSL_VERSION=1.1.0g
RUN wget https://www.openssl.org/source/openssl-${OPENSSL_VERSION}.tar.gz
RUN tar xvfz openssl-${OPENSSL_VERSION}.tar.gz && cd openssl-${OPENSSL_VERSION} && ./config && make && make install
RUN openssl version

ENTRYPOINT ["/usr/local/bin/app"]