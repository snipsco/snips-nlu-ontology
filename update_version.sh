#!/usr/bin/env bash
source ./.travis/common.sh

parseRustVersion() {
    grep -w Cargo.toml -e '^version = ".*' | sed -- 's/version = "//g' | sed -- 's/"//g'
}

NEW_VERSION=$(parseRustVersion)
echo "Updating versions to version ${NEW_VERSION}"
perl -p -i -e "s/^version = \".*\"\$/version = \"$NEW_VERSION\"/g" */Cargo.toml
perl -p -i -e "s/^version = \".*\"\$/version = \"$NEW_VERSION\"/g" */**/*/build.gradle
echo "$NEW_VERSION" > snips-nlu-ontology-ffi/platforms/snips-nlu-ontology-python/snips_nlu_ontology/__version__
