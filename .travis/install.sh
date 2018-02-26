#!/usr/bin/env bash
source .travis/common.sh

echo "Rust build"

export PATH="/usr/local/bin:$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR="$TRAVIS_BUILD_DIR/snips-nlu-ontology-ffi/platforms/snips-nlu-ontology-python/snips-nlu-ontology-rs/target"

perl -p -i -e "s/^snips-nlu-ontology-ffi = .*\$/snips-nlu-ontology-ffi = { path = \"..\/..\/..\" \}/g" */**/**/*/Cargo.toml
cargo build --all  || die "Rust build failed"
