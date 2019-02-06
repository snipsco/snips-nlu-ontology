#!/bin/sh

NEW_VERSION=${1?"usage $0 <new version>"}

echo "Updating versions to version ${NEW_VERSION}"
find . -name "Cargo.toml" -exec perl -p -i -e "s/^version = \".*\"$/version = \"$NEW_VERSION\"/g" {} \;
find . -name "build.gradle" -exec perl -p -i -e "s/^version = \".*\"$/version = \"$NEW_VERSION\"/g" {} \;
