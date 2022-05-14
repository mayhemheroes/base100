# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive
## Install build dependencies.
RUN apt-get update 
RUN apt-get install -y cmake clang
RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /base100/
WORKDIR /base100/

## TODO: ADD YOUR BUILD INSTRUCTIONS HERE.

WORKDIR /base100/fuzz/
RUN cargo fuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly

## TODO: Change <Path in Builder Stage>
COPY --from=builder /base100/fuzz/target/x86_64-unknown-linux-gnu/release/convert /
