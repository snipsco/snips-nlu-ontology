#!/bin/bash
set -ev

source .travis/common.sh

echo "Running rust tests..."
export PATH="$HOME/.cargo/bin:$PATH"
cargo test --all

echo "Running python tests..."
cd platforms/snips-nlu-ontology-python
python -m pip install tox
tox
