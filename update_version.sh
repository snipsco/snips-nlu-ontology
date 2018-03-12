#!/usr/bin/env bash
parseRustVersion() {
    grep -w snips-nlu-ontology/Cargo.toml -e '^version = ".*' | sed -- 's/version = "//g' | sed -- 's/"//g'
}

NEW_VERSION=$(parseRustVersion)
echo "Updating versions to version ${NEW_VERSION}"
find . -name "Cargo.toml" -exec perl -p -i -e "s/^version = \".*\"$/version = \"$NEW_VERSION\"/g" {} \;
find . -name "build.gradle" -exec perl -p -i -e "s/^version = \".*\"$/version = \"$NEW_VERSION\"/g" {} \;
perl -p -i -e "s/snips-nlu-ontology\", tag = \".*\"/snips-nlu-ontology\", tag = \"$NEW_VERSION\"/g" \
    platforms/snips-nlu-ontology-python/snips-nlu-ontology-rs/Cargo.toml
echo "$NEW_VERSION" > platforms/snips-nlu-ontology-python/snips_nlu_ontology/__version__
