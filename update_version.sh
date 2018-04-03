#!/usr/bin/env bash

NEW_VERSION=${1?"usage $0 <new version>"}

echo "Updating versions to version ${NEW_VERSION}"
find . -name "Cargo.toml" -exec perl -p -i -e "s/^version = \".*\"$/version = \"$NEW_VERSION\"/g" {} \;
find . -name "build.gradle" -exec perl -p -i -e "s/^version = \".*\"$/version = \"$NEW_VERSION\"/g" {} \;


if [[ "${NEW_VERSION}" == "${NEW_VERSION/-SNAPSHOT/}" ]]
then
    perl -p -i -e "s/snips-nlu-ontology\", tag = \".*\"/snips-nlu-ontology\", tag = \"$NEW_VERSION\"/g" \
        platforms/snips-nlu-ontology-python/snips-nlu-ontology-rs/Cargo.toml
    perl -p -i -e "s/snips-nlu-ontology\", branch = \".*\"/snips-nlu-ontology\", tag = \"$NEW_VERSION\"/g" \
        platforms/snips-nlu-ontology-python/snips-nlu-ontology-rs/Cargo.toml
else
    perl -p -i -e "s/snips-nlu-ontology\", branch = \".*\"/snips-nlu-ontology\", branch = \"develop\"/g" \
        platforms/snips-nlu-ontology-python/snips-nlu-ontology-rs/Cargo.toml
    perl -p -i -e "s/snips-nlu-ontology\", tag = \".*\"/snips-nlu-ontology\", branch = \"develop\"/g" \
        platforms/snips-nlu-ontology-python/snips-nlu-ontology-rs/Cargo.toml
fi

echo "$NEW_VERSION" > platforms/snips-nlu-ontology-python/snips_nlu_ontology/__version__
