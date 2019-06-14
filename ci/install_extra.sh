#!/usr/bin/env bash

set -ex

rustup component add rustfmt clippy
rustfmt --version
cargo clippy --version
