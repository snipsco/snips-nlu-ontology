#!/bin/sh -e

SCRIPTS_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ROOT_DIR=$SCRIPTS_DIR/..

pushd $ROOT_DIR # Go where Cargo.lock is

cbindgen \
    --crate snips-nlu-ontology-ffi \
    --output snips-nlu-ontology-ffi/platforms/snips-nlu-ontology-c/libsnips_nlu_ontology.h

popd
