#!/usr/bin/env bash

set -ex

cargo fmt --all -- --check
cargo clippy --all-targets --all-features --fix --allow-dirty -- -D warnings

cargo build
cargo test

cargo test --no-default-features --features=plain
cargo test --no-default-features --features=with_tone
cargo test --no-default-features --features=with_tone_num
cargo test --no-default-features --features=with_tone_num_end

cargo test --no-default-features --features=plain,heteronym
cargo test --no-default-features --features=with_tone,heteronym
cargo test --no-default-features --features=with_tone_num,heteronym
cargo test --no-default-features --features=with_tone_num_end,heteronym
