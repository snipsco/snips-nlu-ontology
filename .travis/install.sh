#!/usr/bin/env bash
source .travis/common.sh

echo "Rust build"

export PATH="/usr/local/bin:$HOME/.cargo/bin:$PATH"

cargo build --all  || die "Rust build failed"
