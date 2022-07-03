FROM rust:1.62 as build

# create a new empty shell project
RUN USER=root cargo new --bin arb_data
WORKDIR /arb_data

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/arb_data*
RUN cargo build --release

# our final base
FROM rust:latest

# copy the build artifact from the build stage
COPY --from=build /arb_data/target/release/arb_data .

# set the startup command to run your binary
CMD ["./arb_data"]