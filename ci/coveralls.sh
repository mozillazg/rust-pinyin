#!/usr/bin/env bash

if [[ "$TRAVIS_RUST_VERSION" == stable ]]; then
  curl -sL https://github.com/xd009642/tarpaulin/releases/download/0.8.3/cargo-tarpaulin-0.8.3-travis.tar.gz | \
    tar xvz -C $HOME/.cargo/bin
  cargo tarpaulin --ciserver travis-ci --coveralls $TRAVIS_JOB_ID \
    --exclude-files 'tests/*' --exclude-files 'coverage-check/*'
fi
