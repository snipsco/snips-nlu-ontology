#!/bin/bash
set -ev

cargo build --all
cargo test --all

if [[ "$KOTLIN_TESTS" == "true" ]]; then
  cd platforms/kotlin
  ./gradlew -Pdebug build --info
  cd ../..
fi
