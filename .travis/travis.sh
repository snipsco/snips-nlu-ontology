#!/bin/bash
set -ev

# Install Rust
if [[ -z ${TRAVIS_RUST_VERSION+w} ]]; then
  curl https://sh.rustup.rs -sSf | bash -s -- -y
fi

export PATH="$HOME/.cargo/bin:$PATH"

cargo build --all
cargo test --all

if [[ "$KOTLIN_TESTS" == "true" ]]; then
  cd platforms/kotlin
  ./gradlew -Pdebug build --info
  cd ../..
fi
