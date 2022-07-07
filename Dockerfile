# fails at cc then time_macros
# FROM rust:1.62-alpine3.16 AS build

# COPY . /app

# WORKDIR /app

# RUN cargo build --release

# FROM alpine as runtime

# COPY --from=build /app/target/release/arb_data /
# EXPOSE 8000
# CMD ["./arb_data"]

#---------------------------------------------------------------------------

# FROM rust:alpine AS build

# COPY . /app

# WORKDIR /app

# RUN apk add --update openssl && \
#     rm -rf /var/cache/apk/*

# RUN cargo build --release

# FROM alpine as runtime

# COPY --from=build /app/target/release/arb_data /
# EXPOSE 8000
# CMD ["./arb_data"]

#---------------------------------------------------------------------------

# https://github.com/GoogleContainerTools/distroless/blob/main/examples/rust/Dockerfile
# builds but then 126 error when running container.....bleh
# FROM rust:1.62 as build-env
# WORKDIR /app
# COPY . /app
# RUN cargo build --release

# FROM gcr.io/distroless/cc
# COPY --from=build-env /app/target/release/arb_data /
# CMD ["./arb_data"]

# ---------------------------------------------------------------------------
# remember to add folders and files as per the link bellow
# https://github.com/nbittich/karsher/blob/master/Dockerfile

# FROM rust:alpine as builder


# WORKDIR /app
# RUN cargo new arb_data
# WORKDIR /app/arb_data

# COPY rust-toolchain.toml .

# COPY ./Cargo.toml ./Cargo.lock ./

# COPY .cargo/config .cargo/config

# ENV RUSTFLAGS='-C link-arg=-s'

# RUN cargo build --release 
# RUN rm -rf ./src

# COPY ./src/ ./src

# RUN rm ./target/x86_64-unknown-linux-musl/release/deps/arb_data*

# RUN cargo build --release 

# FROM alpine

# ENV RUST_LOG=info

# VOLUME /root/.local/share

# COPY --from=builder  /app/arb_data/target/x86_64-unknown-linux-musl/release/arb_data .
# CMD [ "/arb_data" ]

#---------------------------------------------------------------------------
# fails at cc then time macros on the rust compile.....bleh
# FROM rust:alpine AS build

# COPY . /app

# WORKDIR /app

# RUN cargo build --release

# # FROM rust:alpine as runtime

# # COPY --from=build /app/target/release/arb_data /
# EXPOSE 8000
# CMD ["./arb_data"]

#---------------------------------------------------------------------------
# https://github.com/kpcyrd/mini-docker-rust/blob/main/Dockerfile

FROM rust:1.62.0-alpine3.16
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
# if needed, add additional dependencies here
RUN apk add --no-cache musl-dev

# set the workdir and copy the source into it
WORKDIR /app
COPY ./ /app
# do a release build
RUN cargo build --release
RUN strip target/release/arb_data

# use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.16
# if needed, install additional dependencies here
RUN apk add --no-cache libgcc
# copy the binary into the final image
COPY --from=0 /app/target/release/arb_data .
# set the binary as entrypoint
EXPOSE 8000
ENTRYPOINT ["/arb_data"]