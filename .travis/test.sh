#!/usr/bin/env bash
source .travis/common.sh

echo "Rust tests..."
export PATH="/usr/local/bin:$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR="$TRAVIS_BUILD_DIR/snips-nlu-ontology-ffi/platforms/snips-nlu-ontology-python/snips-nlu-ontology-rs/target"
cargo test --all || die "Rust tests failed"

echo "Python tests..."
cd snips-nlu-ontology-ffi/platforms/snips-nlu-ontology-python
python -m pip install tox
tox || die "Python tests failed"
