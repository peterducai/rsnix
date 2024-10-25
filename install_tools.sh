#!/bin/bash

rustup override set nightly
rustup component add llvm-tools-preview
cargo install cargo-binutils
rustup target add x86_64-unknown-linux-gnu