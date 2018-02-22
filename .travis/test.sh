#!/usr/bin/env bash
source .travis/common.sh

echo "Rust tests..."
export PATH="/usr/local/bin:$HOME/.cargo/bin:$PATH"
cargo test --all || die "Rust tests failed"

echo "Python tests..."
cd snips-nlu-ontology-ffi/platforms/snips-nlu-ontology-python
python -m pip install tox
tox || die "Python tests failed"
