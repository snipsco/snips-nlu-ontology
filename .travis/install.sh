#!/usr/bin/env bash
source .travis/common.sh

echo "Rust build"

export PATH="/usr/local/bin:$HOME/.cargo/bin:$PATH"

perl -p -i -e "s/^snips-nlu-ontology-ffi = .*\$/snips-nlu-ontology-ffi = { path = \"..\/..\/..\" \}/g" */**/**/*/Cargo.toml
cargo build --all  || die "Rust build failed"
