#!/bin/bash
set -ev

source .travis/common.sh

echo '
[workspace]
members = [
    "snips-nlu-ontology",
    "snips-nlu-ontology-parsers",
    "snips-nlu-ontology-ffi-macros",
    "snips-nlu-ontology-ffi",
    "snips-nlu-ontology-ffi-with-parsers",
    "snips-nlu-ontology-doc",
    "platforms/snips-nlu-ontology-python/snips-nlu-ontology-rs"
]' > Cargo.toml

perl -p -i -e "s/^snips-nlu-ontology-ffi = .*\$/snips-nlu-ontology-ffi = { path = \"..\/..\/..\/snips-nlu-ontology-ffi-with-parsers\" \}/g" \
    platforms/snips-nlu-ontology-python/snips-nlu-ontology-rs/Cargo.toml
