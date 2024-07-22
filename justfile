default:
  @just --list

run:
    cargo run

run-release:
    cargo run --release

build:
    cargo install cargo-bundle
    cargo bundle --release
    rm -rf ./bundle
    mv target/release/bundle ./
    