# rust-rocket-mongo-example

### About
  - This is a work in progress

### Pre-requisites
 - [Install Rust](https://www.rust-lang.org/en-US/install.html)
 - Rustup to nightly build required
  - curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
  - rustup update
  - rustup install nightly
    - rustup run nightly rustc --version
    - rustup default nightly

### Environment Variables

    export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
    export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
    export DEP_OPENSSL_INCLUDE=`brew --prefix openssl`/include

  - MONGO_ADDRESS

## Run
 - cargo run

## Build
  - cargo build

## Release
 - cargo build --release

## Executable
 - ./target/release/rust-rocket-mongo-example